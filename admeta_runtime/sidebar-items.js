window.SIDEBAR_ITEMS = {"constant":[["DAYS",""],["HOURS",""],["MILLISECS_PER_BLOCK","This determines the average expected block time that we are targeting. Blocks will be produced at a minimum duration defined by `SLOT_DURATION`. `SLOT_DURATION` is picked up by `pallet_timestamp` which is in turn picked up by `pallet_aura` to implement `fn slot_duration()`."],["MINUTES",""],["SLOT_DURATION",""],["VERSION",""],["WASM_BINARY",""],["WASM_BINARY_BLOATY",""],["WEIGHT_PER_SECOND",""]],"derive":[["RuntimeDebug",""]],"enum":[["BalancesCall","Contains one variant per dispatchable that can be called by an extrinsic."],["Call",""],["Event",""],["OriginCaller",""],["TimestampCall","Contains one variant per dispatchable that can be called by an extrinsic."]],"fn":[["native_version","The version information used to identify this runtime when compiled natively."]],"macro":[["construct_runtime","Construct a runtime, with the given name and the given pallets."],["parameter_types","Create new implementations of the `Get` trait."]],"mod":[["api",""],["constants",""],["opaque","Opaque types. These are used by the CLI to instantiate machinery that don’t need to know the specifics of the runtime. They can then be made to be agnostic over specific formats of data like extrinsics, allowing for them to continue syncing the network through upgrades to even the core data structures."]],"struct":[["AdDepositBase",""],["AdDepositPerByte",""],["BlockExecutionWeight","Importing a block with 0 txs takes ~5 ms"],["BlockHashCount",""],["BlockLength",""],["BlockWeights","We allow for 2 seconds of compute with a 6 second average block time."],["Burn",""],["CooloffPeriod",""],["CouncilMaxMembers",""],["CouncilMaxProposals",""],["CouncilMotionDuration",""],["EnactmentPeriod",""],["ExistentialDeposit",""],["ExtrinsicBaseWeight","Executing 10,000 System remarks (no-op) txs takes ~1.26 seconds -> ~125 µs per tx"],["FastTrackVotingPeriod",""],["GenesisConfig",""],["IdentityFee","Implementor of `WeightToFeePolynomial` that maps one unit of weight to one unit of fee."],["InstantAllowed",""],["LaunchPeriod",""],["MaxAdDataLength",""],["MaxApprovals",""],["MaxAuthorities",""],["MaxLocks",""],["MaxProposals",""],["MaxScheduledPerBlock",""],["MaxVotes",""],["MaximumSchedulerWeight",""],["MinimumDeposit",""],["MinimumPeriod",""],["OperationalFeeMultiplier",""],["Origin","The runtime origin type represanting the origin of a call."],["OriginPrivilegeCmp","Used the compare the privilege of an origin inside the scheduler."],["PalletId","A pallet identifier. These are per pallet and should be stored in a registry somewhere."],["PalletInfo","Provides an implementation of `PalletInfo` to provide information about the pallet setup in the runtime."],["Perbill","A fixed point representation of a number in the range [0, 1]."],["Permill","A fixed point representation of a number in the range [0, 1]."],["PreimageByteDeposit",""],["ProposalBond",""],["ProposalBondMinimum",""],["RocksDbWeight","By default, Substrate uses RocksDB, so this will be the weight used throughout the runtime."],["Runtime",""],["RuntimeApi",""],["RuntimeApiImpl","Implements all runtime apis for the client side."],["SS58Prefix",""],["SpendPeriod",""],["StorageInfo","Metadata about storage from the runtime."],["TechnicalMaxMembers",""],["TechnicalMaxProposals",""],["TechnicalMotionDuration",""],["TransactionByteFee",""],["TreasuryPalletId",""],["Version",""],["VotingPeriod",""]],"trait":[["BuildStorage","Complex storage builder stuff."],["KeyOwnerProofSystem","Something which can compute and check proofs of a historical key owner and return full identification data of that key owner."],["PrivilegeCmp","Something that can compare privileges of two origins."],["Randomness","A trait that is able to provide randomness."],["StorageValue","A trait for working with macro-generated storage values under the substrate storage API."]],"type":[["AccountId","Some way of identifying an account on the chain. We intentionally make it equivalent to the public key of our transaction signing scheme."],["Ad",""],["Address","The address format for describing accounts."],["AllModules","All modules included in the runtime as a nested tuple of types. Excludes the System pallet."],["AllModulesWithSystem","All modules included in the runtime as a nested tuple of types."],["AllPallets","All pallets included in the runtime as a nested tuple of types. Excludes the System pallet."],["AllPalletsWithSystem","All pallets included in the runtime as a nested tuple of types."],["Aura",""],["AuraConfig",""],["Balance","Balance of an account."],["Balances",""],["BalancesConfig",""],["Block","Block type as expected by this runtime."],["BlockNumber","An index to a block."],["Council",""],["CouncilCollective",""],["CouncilConfig",""],["CouncilMembership",""],["CouncilMembershipConfig",""],["Democracy",""],["DemocracyConfig",""],["Executive","Executive: handles dispatch to the various modules."],["Grandpa",""],["GrandpaConfig",""],["Hash","A hash of some data used by the chain."],["Header","Block header type as expected by this runtime."],["Index","Index of a transaction in the chain."],["RandomnessCollectiveFlip",""],["Scheduler",""],["SchedulerConfig",""],["Signature","Alias to 512-bit hash when used in the context of a transaction signature on the chain."],["SignedExtra","The SignedExtension to the basic transaction logic."],["Sudo",""],["SudoConfig",""],["System",""],["SystemConfig",""],["TechnicalCommittee",""],["TechnicalCommitteeConfig",""],["TechnicalCommitteeMembership",""],["TechnicalCommitteeMembershipConfig",""],["Timestamp",""],["TransactionPayment",""],["TransactionPaymentConfig",""],["Treasury",""],["TreasuryConfig",""],["UncheckedExtrinsic","Unchecked extrinsic type as expected by this runtime."],["User",""],["Weight","Numeric range of a transaction weight."]]};