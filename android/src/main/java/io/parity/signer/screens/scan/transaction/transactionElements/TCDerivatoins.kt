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
import io.parity.signer.components.toImageContent
import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.pink300
import io.parity.signer.ui.theme.textTertiary
import io.parity.signer.uniffi.DerivedKeyError
import io.parity.signer.uniffi.DerivedKeyStatus
import io.parity.signer.uniffi.SeedKeysPreview

@Composable
fun TCDerivationsFull(payload: List<String>) {
	Column {
//error states ios/NativeSigner/Cards/TransactionCards/TCDerivations.swift:104
		Text(
			stringResource(R.string.transaction_field_import_derivations),
			style = SignerTypeface.BodyL,
			color = MaterialTheme.colors.textTertiary
		)
		for (record in payload) {
			Text(
				record,
				style = SignerTypeface.BodyM,
				color = MaterialTheme.colors.pink300
			)
		}
	}
}


@Composable
fun TCDerivationsFull(model: DerivedKeysSetModel) {
	Column() {

	}
}


/**
 * Local version of shared [SeedKeysPreview] class
 */
data class DerivedKeysSetModel(
	val seedName: String,
	val address: String,
	val errors: Errors,
	val keys: List<DerivedKeyModel>
) {
	data class Errors(
		val isNetworkMissing: Boolean,
		val isKeySetMissing: Boolean,
		val isPathInBadFormat: Boolean,
	)

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
				errors = Errors(
					isNetworkMissing = true,
					isKeySetMissing = false,
					isPathInBadFormat = true,
				),
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
		errors = DerivedKeysSetModel.Errors(
			isNetworkMissing = derivedKeys
				.asSequence()
				.map { it.status }
				.filterIsInstance<DerivedKeyStatus.Invalid>()
				.flatMap { it.errors }
				.any { it is DerivedKeyError.NetworkMissing },
			isKeySetMissing = derivedKeys
				.asSequence()
				.map { it.status }
				.filterIsInstance<DerivedKeyStatus.Invalid>()
				.flatMap { it.errors }
				.any { it is DerivedKeyError.KeySetMissing },
			isPathInBadFormat = derivedKeys
				.asSequence()
				.map { it.status }
				.filterIsInstance<DerivedKeyStatus.Invalid>()
				.flatMap { it.errors }
				.any { it is DerivedKeyError.BadFormat },
		),
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
//todo import derivations           updateErrorStates(value)


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
private fun PreviewTCDerivations() {
	SignerNewTheme {
		Column {
			TCDerivationsFull(payload = listOf("Derivation 1", "Derivation 2"))
//			SignerDivider()
		}
	}
}


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
		val model = DerivedKeysSetModel.createStub()
		Column {
			TCDerivationsFull(model = model)
		}
	}
}
