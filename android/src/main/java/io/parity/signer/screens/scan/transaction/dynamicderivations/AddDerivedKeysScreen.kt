package io.parity.signer.screens.scan.transaction.dynamicderivations

import android.content.res.Configuration
import androidx.activity.compose.BackHandler
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalInspectionMode
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.NotificationFrameTextAlert
import io.parity.signer.components.base.NotificationFrameTextImportant
import io.parity.signer.components.base.PrimaryButtonWide
import io.parity.signer.components.base.ScreenHeaderClose
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.components.qrcode.AnimatedQrKeysInfo
import io.parity.signer.components.qrcode.EmptyAnimatedQrKeysProvider
import io.parity.signer.components.qrcode.EmptyQrCodeProvider
import io.parity.signer.domain.Callback
import io.parity.signer.domain.KeyModel
import io.parity.signer.domain.getData
import io.parity.signer.screens.keysetdetails.items.KeyDerivedItem
import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.fill6
import io.parity.signer.uniffi.DdDetail
import io.parity.signer.uniffi.DdKeySet
import io.parity.signer.uniffi.DdPreview
import io.parity.signer.uniffi.QrData

@Composable
internal fun AddDerivedKeysScreen(
	model: DdPreview,
	modifier: Modifier = Modifier,
	onBack: Callback,
	onDone: Callback,
) {
	BackHandler(onBack = onBack)
	Column(
		modifier = modifier.verticalScroll(rememberScrollState()),
	) {
		ScreenHeaderClose(
			onClose = onBack,
			title = null,
			modifier = Modifier.padding(horizontal = 8.dp)
		)
		Text(
			text = stringResource(R.string.add_derived_keys_screen_title),
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.TitleL,
			modifier = Modifier.padding(horizontal = 24.dp),
		)
		Text(
			text = stringResource(R.string.add_derived_keys_screen_subtitle),
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.BodyL,
			modifier = Modifier
				.padding(horizontal = 24.dp)
				.padding(top = 8.dp, bottom = 20.dp),
		)

		if (model.isSomeAlreadyImported) {
			NotificationFrameTextImportant(
				message = stringResource(R.string.dymanic_derivation_error_some_already_imported),
				withBorder = false,
				textColor = MaterialTheme.colors.primary,
				modifier = Modifier
					.padding(bottom = 8.dp)
					.padding(horizontal = 16.dp),
			)
		}
		if (model.isSomeNetworkMissing) {
			NotificationFrameTextImportant(
				message = stringResource(R.string.dymanic_derivation_error_some_network_missing),
				withBorder = false,
				textColor = MaterialTheme.colors.primary,
				modifier = Modifier
					.padding(bottom = 8.dp)
					.padding(horizontal = 16.dp),
			)
		}

		KeysetItemDerivedItem(model.keySet)

		Text(
			text = stringResource(R.string.add_derived_keys_screen_scan_qr_code),
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.BodyL,
			modifier = Modifier
				.padding(horizontal = 24.dp)
				.padding(top = 16.dp, bottom = 20.dp),
		)
		if (LocalInspectionMode.current) {
			AnimatedQrKeysInfo(
				input = Unit,
				provider = EmptyAnimatedQrKeysProvider(),
				modifier = Modifier.padding(horizontal = 24.dp)
			)
		} else {
			AnimatedQrKeysInfo<List<List<UByte>>>(
				input = model.qr.map { it.getData() },
				provider = EmptyQrCodeProvider(),
				modifier = Modifier.padding(horizontal = 24.dp)
			)
		}

		PrimaryButtonWide(
			label = stringResource(R.string.generic_done),
			isEnabled = model.keySet.derivations.isNotEmpty(),
			modifier = Modifier.padding(horizontal = 24.dp, vertical = 32.dp),
			onClicked = onDone,
		)
	}
}

@Composable
private fun KeysetItemDerivedItem(model: DdKeySet) {
	if (model.derivations.isEmpty()) return //empty keyset - don't show whole section

	Column(
		modifier = Modifier
			.padding(horizontal = 16.dp, vertical = 4.dp)
			.background(
				MaterialTheme.colors.fill6,
				RoundedCornerShape(dimensionResource(id = R.dimen.plateDefaultCornerRadius))
			)
	) {
		Row(
			verticalAlignment = Alignment.CenterVertically
		) {
			Text(
				text = model.seedName,
				color = MaterialTheme.colors.primary,
				style = SignerTypeface.TitleS,
				modifier = Modifier.padding(16.dp)
			)
			Spacer(modifier = Modifier.weight(1f))
		}
		model.derivations.forEach { key ->
			SignerDivider()
			KeyDerivedItem(model = key.toKeyModel(), key.networkLogo, onClick = null)
		}
	}
}


private fun DdDetail.toKeyModel() = KeyModel(
	identicon = identicon,
	addressKey = "",
	seedName = "",
	base58 = base58,
	hasPwd = false,
	path = path,
	secretExposed = false,
)

private fun ddPreviewcreateStub(): DdPreview = DdPreview(
	qr = listOf(
		QrData.Regular(PreviewData.exampleQRData),
	),
	keySet = DdKeySet(
		seedName = "My special keyset",
		derivations = listOf(
			ddDetailcreateStub(),
			ddDetailcreateStub(),
		),
	),
	isSomeAlreadyImported = false,
	isSomeNetworkMissing = true,
)

private fun ddDetailcreateStub(): DdDetail = DdDetail(
	base58 = "5F3sa2TJAWMqDhXG6jhV4N8ko9SxwGy8TpaNS1repo5EYjQX",
	path = "//polkadot//path2",
	networkLogo = "westend",
	networkSpecsKey = "sdfsdfgdfg",
	identicon = PreviewData.Identicon.dotIcon,
)


@Preview(
	name = "light", group = "general", uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true, backgroundColor = 0xFFFFFFFF,
)
@Preview(
	name = "dark", group = "general",
	uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun PreviewAddDerivedKeysScreen() {
	SignerNewTheme {
		AddDerivedKeysScreen(
			model = ddPreviewcreateStub(),
			onBack = {},
			onDone = {},
		)
	}
}
