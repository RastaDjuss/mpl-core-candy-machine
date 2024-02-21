/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import { Serializer, struct, u64 } from '@metaplex-foundation/umi/serializers';

/**
 * Guard that stop the mint once the specified amount of items
 * redeenmed is reached.
 */

export type RedeemedAmount = { maximum: bigint };

export type RedeemedAmountArgs = { maximum: number | bigint };

export function getRedeemedAmountSerializer(): Serializer<
  RedeemedAmountArgs,
  RedeemedAmount
> {
  return struct<RedeemedAmount>([['maximum', u64()]], {
    description: 'RedeemedAmount',
  }) as Serializer<RedeemedAmountArgs, RedeemedAmount>;
}
