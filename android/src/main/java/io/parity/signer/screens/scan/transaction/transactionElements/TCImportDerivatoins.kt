package io.parity.signer.screens.scan.transaction.transactionElements

import android.content.res.Configuration
import androidx.compose.foundation.layout.Column
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import io.parity.signer.R
import io.parity.signer.components.ImageContent
import io.parity.signer.components.base.NotificationFrameText
import io.parity.signer.components.toImageContent
import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.pink300
import io.parity.signer.ui.theme.textTertiary
import io.parity.signer.uniffi.Card
import io.parity.signer.uniffi.DerivedKeyError
import io.parity.signer.uniffi.DerivedKeyStatus
import io.parity.signer.uniffi.SeedKeysPreview

@Composable
fun TCImportDerivationsFull(model: ImportDerivationsModel) {
	Column {
		TCDerivationsErrors(model.errors)
	}
}


@Composable
fun TCDerivationsSingle(model: DerivedKeysSetModel) {
	Column() {


	}
}

@Composable
private fun TCDerivationsErrors(errors: ImportDerivationsModel.Errors) {
	Column() {
		if (errors.isKeySetMissing) {
			NotificationFrameText(R.string.import_derivations_error_key_missing, withBorder = false)
		}
		if (errors.isNetworkMissing) {
			NotificationFrameText(R.string.import_derivations_error_network_missing, withBorder = false)
		}
		if (errors.isPathInBadFormat) {
			NotificationFrameText(R.string.import_derivations_error_path_bad_format, withBorder = false)
		}
		if (errors.keysAlreadyExist) {
			NotificationFrameText(R.string.import_derivations_error_keys_already_exist, withBorder = false)
		}
	}
}


/**
 * Local version of shared [TCDerivedCard] class
 */
data class ImportDerivationsModel(
	val keySets: List<DerivedKeysSetModel>,
	val errors: Errors,
) {
	data class Errors(
		val isNetworkMissing: Boolean,
		val isKeySetMissing: Boolean,
		val isPathInBadFormat: Boolean,
		val keysAlreadyExist: Boolean,
	)

	companion object {
		fun createStub(): ImportDerivationsModel = ImportDerivationsModel(
			keySets = listOf(DerivedKeysSetModel.createStub()),
			errors = Errors(
				isNetworkMissing = true,
				isKeySetMissing = false,
				isPathInBadFormat = true,
				keysAlreadyExist = true,
			),
		)
	}
}

fun Card.DerivationsCard.toImportDerivationsModel(): ImportDerivationsModel =
	ImportDerivationsModel(
		keySets = f.map { it.toDerivedKeysSetModel() },
		errors = ImportDerivationsModel.Errors(
			isNetworkMissing = f
				.asSequence()
				.flatMap { it.derivedKeys }
				.map { it.status }
				.filterIsInstance<DerivedKeyStatus.Invalid>()
				.flatMap { it.errors }
				.any { it is DerivedKeyError.NetworkMissing },
			isKeySetMissing = f
				.asSequence()
				.flatMap { it.derivedKeys }
				.map { it.status }
				.filterIsInstance<DerivedKeyStatus.Invalid>()
				.flatMap { it.errors }
				.any { it is DerivedKeyError.KeySetMissing },
			isPathInBadFormat = f
				.asSequence()
				.flatMap { it.derivedKeys }
				.map { it.status }
				.filterIsInstance<DerivedKeyStatus.Invalid>()
				.flatMap { it.errors }
				.any { it is DerivedKeyError.BadFormat },
			keysAlreadyExist = f
				.asSequence()
				.flatMap { it.derivedKeys }
				.map { it.status }
				.any { it is DerivedKeyStatus.AlreadyExists },
		),
	)

/**
 * Local version of shared [SeedKeysPreview] class
 */
data class DerivedKeysSetModel(
	val seedName: String,
	val address: String,
	val keys: List<DerivedKeyModel>
) {

	data class DerivedKeyModel(
		val identicon: ImageContent,
		val derivationPath: String,
		val hadPwd: Boolean,
		val address: String,
		val networkTitle: String?
	)

	companion object {
		fun createStub(): DerivedKeysSetModel =
			DerivedKeysSetModel(
				seedName = "Derivation 1",
				address = "long address",
				keys = listOf(
					DerivedKeyModel(
						identicon = PreviewData.exampleIdenticonPng,
						derivationPath = "//kusama",
						hadPwd = false,
						address = "address",
						networkTitle = "Kusama",
					),
					DerivedKeyModel(
						identicon = PreviewData.exampleIdenticonPng,
						derivationPath = "//westendMain",
						hadPwd = true,
						address = "GD5434gFGFD543Dgdf",
						networkTitle = "Westend",
					),
					DerivedKeyModel(
						identicon = PreviewData.exampleIdenticonPng,
						derivationPath = "//polka",
						hadPwd = false,
						address = "address",
						networkTitle = "Polkadot",
					),
					DerivedKeyModel(
						identicon = PreviewData.exampleIdenticonPng,
						derivationPath = "//polkadot//parachains",
						hadPwd = false,
						address = "address",
						networkTitle = null,
					),
					DerivedKeyModel(
						identicon = PreviewData.exampleIdenticonPng,
						derivationPath = "//polkadot//staking",
						hadPwd = false,
						address = "address",
						networkTitle = null,
					),
				),
			)
	}
}

fun SeedKeysPreview.toDerivedKeysSetModel(): DerivedKeysSetModel =
	DerivedKeysSetModel(
		seedName = name,
		address = multisigner.firstOrNull() ?: "",
		keys = derivedKeys
			.filter { it.status == DerivedKeyStatus.Importable }
			.filter { it.hasPwd != null }
			.map {
				DerivedKeysSetModel.DerivedKeyModel(
					identicon = it.identicon.toImageContent(),
					derivationPath = it.derivationPath ?: "",
					hadPwd = it.hasPwd == true,
					address = it.address,
					networkTitle = it.networkTitle,
				)
			}
	)


///** todo import derivations delete?
//// * Local copy of shared [DerivationsCard] class
//// */
//data class DerivationCardModel(val seedKeys: List<SeedKeysPreviewModel>)
//
//fun Card.DerivationsCard.toDerivationCardModel() = DerivationCardModel(
//	this.f.map { it.toSeedKeysPreviewModel() }
//)
//
///**
// * Local copy of shared [SeedKeysPreview] class
// */
//data class SeedKeysPreviewModel(
//	val name: String,
//	val derivedKeys: List<DerivedKeyPreview>,
////	var `multisigner`: MultiSigner,
//)
//
//fun SeedKeysPreview.toSeedKeysPreviewModel() = SeedKeysPreviewModel(
//	name = name,
//	derivedKeys = derivedKeys
//)
////ios/NativeSigner/Cards/TransactionCards/TCDerivations.swift todo import derivations


@Preview(
	name = "light", group = "themes", uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true, backgroundColor = 0xFFFFFFFF,
)
@Preview(
	name = "dark", group = "themes", uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun PreviewTCDerivationsNew() {
	SignerNewTheme {
		val model = ImportDerivationsModel.createStub()
		Column {
			TCImportDerivationsFull(model = model)
		}
	}
}
