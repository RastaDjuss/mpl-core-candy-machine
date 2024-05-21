use mpl_core::{instructions::TransferV1CpiBuilder, types::UpdateAuthority, Asset};

use super::*;
use crate::{state::GuardType, utils::assert_keys_equal};

/// Guard that charges another Core Asset from a specific collection as payment
/// for the mint.
///
/// List of accounts required:
///
///   0. `[writeable]` Asset address.
///   1. `[]` Collection address.
///   2. `[]` Destination address.

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct AssetPayment {
    pub required_collection: Pubkey,
    pub destination: Pubkey,
}

impl Guard for AssetPayment {
    fn size() -> usize {
        32   // required_collection
        + 32 // destination
    }

    fn mask() -> u64 {
        GuardType::as_mask(GuardType::AssetPayment)
    }
}

impl Condition for AssetPayment {
    fn validate<'info>(
        &self,
        ctx: &mut EvaluationContext,
        _guard_set: &GuardSet,
        _mint_args: &[u8],
    ) -> Result<()> {
        let index = ctx.account_cursor;

        let asset_info = try_get_account_info(ctx.accounts.remaining, index)?;
        let collection_info = try_get_account_info(ctx.accounts.remaining, index + 1)?;
        let destination_info = try_get_account_info(ctx.accounts.remaining, index + 2)?;

        ctx.account_cursor += 3;

        let asset = Asset::try_from(asset_info)?;

        let asset_collection = match asset.base.update_authority {
            UpdateAuthority::Collection(pubkey) => Some(pubkey),
            _ => None,
        };

        if asset_collection.is_none() {
            return err!(CandyGuardError::InvalidNftCollection);
        }

        assert_keys_equal(&asset_collection.unwrap(), &self.required_collection)?;
        assert_keys_equal(&collection_info.key(), &self.required_collection)?;
        assert_keys_equal(&destination_info.key(), &self.destination)?;

        ctx.indices.insert("asset_payment_index", index);

        Ok(())
    }

    fn pre_actions<'info>(
        &self,
        ctx: &mut EvaluationContext,
        _guard_set: &GuardSet,
        _mint_args: &[u8],
    ) -> Result<()> {
        let index = ctx.indices["asset_payment_index"];
        let asset_info = try_get_account_info(ctx.accounts.remaining, index)?;
        let collection_info = try_get_account_info(ctx.accounts.remaining, index + 1)?;
        let destination_info = try_get_account_info(ctx.accounts.remaining, index + 2)?;

        let mut transfer_cpi = TransferV1CpiBuilder::new(&ctx.accounts.mpl_core_program);

        transfer_cpi
            .asset(asset_info)
            .collection(Some(collection_info))
            .new_owner(destination_info)
            .payer(&ctx.accounts.payer)
            .authority(Some(&ctx.accounts.minter));

        transfer_cpi.invoke()?;

        Ok(())
    }
}
