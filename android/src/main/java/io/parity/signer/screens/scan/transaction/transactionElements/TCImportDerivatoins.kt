package io.parity.signer.screens.scan.transaction.transactionElements

import android.content.res.Configuration
import androidx.compose.animation.animateContentSize
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.KeyboardArrowDown
import androidx.compose.material.icons.filled.KeyboardArrowUp
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.ExperimentalComposeUiApi
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.pluralStringResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.IdentIcon
import io.parity.signer.components.ImageContent
import io.parity.signer.components.base.NotificationFrameTextImportant
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.components.sharedcomponents.KeyPath
import io.parity.signer.components.toImageContent
import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.ui.theme.*
import io.parity.signer.uniffi.Card
import io.parity.signer.uniffi.DerivedKeyError
import io.parity.signer.uniffi.DerivedKeyStatus
import io.parity.signer.uniffi.SeedKeysPreview

@Composable
fun TCImportDerivationsFull(model: ImportDerivationsModel) {
	Column {
		TCDerivationsErrors(model.errors)
		Column(
			modifier = Modifier.padding(8.dp),
			verticalArrangement = Arrangement.spacedBy(10.dp)
		) {
			model.keySets.forEach { keySet ->
				TCDerivationsSingle(keySet)
			}
		}
	}
}


@OptIn(ExperimentalComposeUiApi::class)
@Composable
private fun TCDerivationsSingle(model: DerivedKeysSetModel) {
	val outerShape =
		RoundedCornerShape(dimensionResource(id = R.dimen.qrShapeCornerRadius))
	val innerShape =
		RoundedCornerShape(dimensionResource(id = R.dimen.innerFramesCornerRadius))
	Column(
		modifier = Modifier
			.fillMaxWidth(1f)
			.background(MaterialTheme.colors.fill6, outerShape)
			.padding(8.dp),
	) {
		SeedKeyCollapsible(
			seedName = model.seedName,
			base58 = model.address,
			modifier = Modifier.padding(8.dp)
		)
		Text(
			pluralStringResource(
				id = R.plurals.import_derivations_subtitle_keys_imported,
				count = model.keys.size,
				model.keys.size
			),
			color = MaterialTheme.colors.textTertiary,
			style = SignerTypeface.BodyM,
			modifier = Modifier.padding(horizontal = 16.dp, vertical = 8.dp)
		)
		Column(
			modifier = Modifier
				.fillMaxWidth(1f)
				.background(MaterialTheme.colors.fill6, innerShape),
		) {
			model.keys.forEachIndexed { index, derivedKeyModel ->
				SingleKeyElement(derivedKeyModel)
				if (index != model.keys.lastIndex) {
					SignerDivider()
				}
			}
		}
	}
}

@Composable
private fun SingleKeyElement(key: DerivedKeysSetModel.DerivedKeyModel) {
	val innerShape =
		RoundedCornerShape(dimensionResource(id = R.dimen.innerFramesCornerRadius))
	Column(
		modifier = Modifier.padding(16.dp),
		verticalArrangement = Arrangement.spacedBy(8.dp),
	) {
		Row() {
			IdentIcon(key.identicon, size = 16.dp)
			Spacer(modifier = Modifier.padding(end = 8.dp))
			KeyPath(path = key.derivationPath, hasPassword = key.hadPwd)
		}
		Text(
			text = key.address,
			color = MaterialTheme.colors.textTertiary,
			style = SignerTypeface.BodyM,
		)
		if (key.networkTitle != null) {
			Text(
				text = key.networkTitle,
				color = MaterialTheme.colors.textTertiary,
				style = SignerTypeface.CaptionM,
				modifier = Modifier
					.background(MaterialTheme.colors.fill12, innerShape)
					.padding(horizontal = 8.dp, vertical = 2.dp),
			)
		}
	}
}

@Composable
private fun SeedKeyCollapsible(
	seedName: String, base58: String, modifier: Modifier = Modifier
) {
	val expanded = remember { mutableStateOf(false) }
	Column(horizontalAlignment = Alignment.Start,
		modifier = modifier
			.clickable { expanded.value = !expanded.value }
			.animateContentSize()) {
		Row(verticalAlignment = Alignment.CenterVertically) {
			Text(
				seedName,
				color = MaterialTheme.colors.primary,
				style = SignerTypeface.TitleS
			)
			Icon(
				imageVector = if (expanded.value) {
					Icons.Default.KeyboardArrowUp
				} else {
					Icons.Default.KeyboardArrowDown
				},
				modifier = Modifier
					.size(24.dp)
					.padding(horizontal = 4.dp),
				contentDescription = stringResource(R.string.description_expand_button),
				tint = MaterialTheme.colors.textTertiary
			)
			Spacer(modifier = Modifier.weight(1f))
		}
		if (expanded.value) {
			Text(
				text = base58,
				color = MaterialTheme.colors.textTertiary,
				style = SignerTypeface.BodyM,
				modifier = Modifier.padding(top = 4.dp),
			)
		}
	}
}

@Composable
private fun TCDerivationsErrors(errors: ImportDerivationsModel.Errors) {
	Column() {
		if (errors.isKeySetMissing) {
			NotificationFrameTextImportant(
				stringResource(id = R.string.import_derivations_error_key_missing),
				withBorder = false
			)
			Spacer(modifier = Modifier.padding(bottom = 8.dp))
		}
		if (errors.isNetworkMissing) {
			NotificationFrameTextImportant(
				stringResource(R.string.import_derivations_error_network_missing),
				withBorder = false
			)
			Spacer(modifier = Modifier.padding(bottom = 8.dp))
		}
		if (errors.isPathInBadFormat) {
			NotificationFrameTextImportant(
				stringResource(R.string.import_derivations_error_path_bad_format),
				withBorder = false
			)
			Spacer(modifier = Modifier.padding(bottom = 8.dp))
		}
		if (errors.keysAlreadyExist) {
			NotificationFrameTextImportant(
				stringResource(R.string.import_derivations_error_keys_already_exist),
				withBorder = false
			)
			Spacer(modifier = Modifier.padding(bottom = 8.dp))
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
			keySets = listOf(
				DerivedKeysSetModel.createStub(),
				DerivedKeysSetModel.createStub()
			),
			errors = Errors(
				isNetworkMissing = true,
				isKeySetMissing = false,
				isPathInBadFormat = false,
				keysAlreadyExist = true,
			),
		)
	}
}

fun Card.DerivationsCard.toImportDerivationsModel(): ImportDerivationsModel =
	ImportDerivationsModel(
		keySets = f.map { it.toDerivedKeysSetModel() },
		errors = ImportDerivationsModel.Errors(
			isNetworkMissing = f.asSequence().flatMap { it.derivedKeys }
				.map { it.status }.filterIsInstance<DerivedKeyStatus.Invalid>()
				.flatMap { it.errors }.any { it is DerivedKeyError.NetworkMissing },
			isKeySetMissing = f.asSequence().flatMap { it.derivedKeys }
				.map { it.status }.filterIsInstance<DerivedKeyStatus.Invalid>()
				.flatMap { it.errors }.any { it is DerivedKeyError.KeySetMissing },
			isPathInBadFormat = f.asSequence().flatMap { it.derivedKeys }
				.map { it.status }.filterIsInstance<DerivedKeyStatus.Invalid>()
				.flatMap { it.errors }.any { it is DerivedKeyError.BadFormat },
			keysAlreadyExist = f.asSequence().flatMap { it.derivedKeys }
				.map { it.status }.any { it is DerivedKeyStatus.AlreadyExists },
		),
	)

/**
 * Local version of shared [SeedKeysPreview] class
 */
data class DerivedKeysSetModel(
	val seedName: String, val address: String, val keys: List<DerivedKeyModel>
) {

	data class DerivedKeyModel(
		val identicon: ImageContent,
		val derivationPath: String,
		val hadPwd: Boolean,
		val address: String,
		val networkTitle: String?
	)

	companion object {
		fun createStub(): DerivedKeysSetModel = DerivedKeysSetModel(
			seedName = "Derivation 1",
			address = "12955s5CP8Fuo1yk2YkJVUKDnZvXD9PKck3nzLZ4A51TT75",
			keys = listOf(
				DerivedKeyModel(
					identicon = PreviewData.Identicon.exampleIdenticonPng,
					derivationPath = "//kusama",
					hadPwd = false,
					address = "12955s5CP8Fuo1yk2YkJVUKDnZvXD9PKck3nzLZ4A51TT75",
					networkTitle = "Kusama",
				),
				DerivedKeyModel(
					identicon = PreviewData.Identicon.exampleIdenticonPng,
					derivationPath = "//westendMain//westendMain//westendMain//westendMain//westendMain//westendMain//westendMain/verylongPath2",
					hadPwd = true,
					address = "GD5434gFGFD543Dgdf",
					networkTitle = "Westend",
				),
				DerivedKeyModel(
					identicon = PreviewData.Identicon.exampleIdenticonPng,
					derivationPath = "//polka",
					hadPwd = false,
					address = "12955s5CP8Fuo1yk2YkJVUKDnZvXD9PKck3nzLZ4A51TT75",
					networkTitle = "Polkadot",
				),
				DerivedKeyModel(
					identicon = PreviewData.Identicon.exampleIdenticonPng,
					derivationPath = "//polkadot//parachains",
					hadPwd = false,
					address = "12955s5CP8Fuo1yk2YkJVUKDnZvXD9PKck3nzLZ4A51TT75",
					networkTitle = null,
				),
				DerivedKeyModel(
					identicon = PreviewData.Identicon.exampleIdenticonPng,
					derivationPath = "//polkadot//staking",
					hadPwd = false,
					address = "12955s5CP8Fuo1yk2YkJVUKDnZvXD9PKck3nzLZ4A51TT75",
					networkTitle = null,
				),
			),
		)
	}
}

fun SeedKeysPreview.toDerivedKeysSetModel(): DerivedKeysSetModel =
	DerivedKeysSetModel(seedName = name,
		address = multisigner.firstOrNull() ?: "",
		keys = derivedKeys.filter { it.status == DerivedKeyStatus.Importable }
			.filter { it.hasPwd != null }.map {
				DerivedKeysSetModel.DerivedKeyModel(
					identicon = it.identicon.toImageContent(),
					derivationPath = it.derivationPath ?: "",
					hadPwd = it.hasPwd == true,
					address = it.address,
					networkTitle = it.networkTitle,
				)
			})


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
