package io.parity.signer

/**
 * List all possible buttons for typesafe navigation
 */
enum class ButtonID {
	Start,
	NavbarLog,
	NavbarScan,
	NavbarKeys,
	NavbarSettings,
	GoBack,
	SelectSeed,
	RightButton,
	Shield,
	SelectKey,
	GoForward,
	Derive,
	Delete,
	NewSeed,
	RecoverSeed,
	NetworkSelector,
	NextUnit,
	PreviousUnit,
	NewKey,
	BackupSeed,
	CheckPassword,
	ChangeNetwork,
	TransactionFetched,
	RemoveNetwork,
	RemoveMetadata,
	RemoveTypes,
	SignNetworkSpecs,
	SignMetadata,
	SignTypes,
	ManageNetworks,
	ViewGeneralVerifier,
	ManageMetadata,
	RemoveKey,
	RemoveSeed,
	ClearLog,
	CreateLogComment,
	ShowLogDetails,
	Swipe,
	LongTap,
	SelectAll,
	Increment,
	ShowDocuments,
	ExportMultiselect;


	fun getName(): String {
		return when (this) {
			NavbarLog -> "Log"
			NavbarScan -> "Scan"
			NavbarKeys -> "Keys"
			NavbarSettings -> "Settings"
			GoBack -> "<"
			else -> ""
		}
	}
}
