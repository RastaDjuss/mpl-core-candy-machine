/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import { Serializer, scalarEnum } from '@metaplex-foundation/umi/serializers';

/** Available guard types. */
export enum GuardType {
  BotTax,
  SolPayment,
  TokenPayment,
  StartDate,
  ThirdPartySigner,
  TokenGate,
  Gatekeeper,
  EndDate,
  AllowList,
  MintLimit,
  NftPayment,
  RedeemedAmount,
  AddressGate,
  NftGate,
  NftBurn,
  TokenBurn,
  FreezeSolPayment,
  FreezeTokenPayment,
  ProgramGate,
  Allocation,
  Token2022Payment,
  SolFixedFee,
  NftMintLimit,
}

export type GuardTypeArgs = GuardType;

export function getGuardTypeSerializer(): Serializer<GuardTypeArgs, GuardType> {
  return scalarEnum<GuardType>(GuardType, {
    description: 'GuardType',
  }) as Serializer<GuardTypeArgs, GuardType>;
}
