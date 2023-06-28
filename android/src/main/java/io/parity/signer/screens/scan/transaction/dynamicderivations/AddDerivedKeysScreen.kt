package io.parity.signer.screens.scan.transaction.dynamicderivations

import android.content.res.Configuration
import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.aspectRatio
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.platform.LocalInspectionMode
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.ScreenHeader
import io.parity.signer.components.base.SecondaryButtonWide
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.domain.Callback
import io.parity.signer.domain.KeyAndNetworkModel
import io.parity.signer.domain.intoImageBitmap
import io.parity.signer.screens.keysetdetails.items.KeyDerivedItem
import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.fill6
import io.parity.signer.uniffi.encodeToQr
import kotlinx.coroutines.runBlocking

@Composable
fun AddDerivedKeysScreen(
	model: AddDerivedKeysModel,
	onBack: Callback,
) {
	Column(
		modifier = Modifier
			.verticalScroll(rememberScrollState()),
	) {
		ScreenHeader(
			onBack = onBack,
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

		model.keysets.forEach { keyset ->
			KeysetItemDerivedItem(keyset)
		}

		Text(
			text = stringResource(R.string.add_derived_keys_screen_scan_qr_code),
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.BodyL,
			modifier = Modifier
				.padding(horizontal = 24.dp)
				.padding(top = 16.dp, bottom = 20.dp),
		)
		Box(
			modifier = Modifier
				.padding(horizontal = 24.dp)
				.fillMaxWidth(1f)
				.aspectRatio(1.1f)
				.background(
					Color.White,
					RoundedCornerShape(dimensionResource(id = R.dimen.qrShapeCornerRadius))
				),
			contentAlignment = Alignment.Center,
		) {
			val isPreview = LocalInspectionMode.current
			val qrImage = remember {
				if (isPreview) {
					PreviewData.exampleQRImage
				} else {
					runBlocking { encodeToQr(model.qrData, false) }
				}
			}
			Image(
				bitmap = qrImage.intoImageBitmap(),
				contentDescription = stringResource(R.string.qr_with_address_to_scan_description),
				contentScale = ContentScale.Fit,
				modifier = Modifier.size(264.dp)
			)
		}

		SecondaryButtonWide(
			label = stringResource(R.string.transaction_action_done),
			withBackground = true,
			modifier = Modifier.padding(horizontal = 24.dp, vertical = 32.dp),
			onClicked = onBack,
		)
	}
}

@Composable
private fun KeysetItemDerivedItem(model: KeysetDerivedModel) {
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
		model.keysets.forEach { keyset ->
			SignerDivider()
			KeyDerivedItem(model = keyset.key, keyset.key.path, onClick = null)
		}
	}
}




data class AddDerivedKeysModel(
	val keysets: List<KeysetDerivedModel>,
	val qrData: List<UByte>,
) {
	companion object {
		fun createStub(): AddDerivedKeysModel = AddDerivedKeysModel(
			listOf(
				KeysetDerivedModel(
					seedName = "My special keyset",
					keysets = listOf(
						KeyAndNetworkModel.createStub(),
						KeyAndNetworkModel.createStub()
					),
				),
				KeysetDerivedModel(
					seedName = "My special keyset2",
					keysets = listOf(
						KeyAndNetworkModel.createStub(),
					),
				)
			),
			qrData = PreviewData.exampleQRData,
		)
	}
}

data class KeysetDerivedModel(
	val seedName: String,
	val keysets: List<KeyAndNetworkModel>,
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
			model = AddDerivedKeysModel.createStub(),
			onBack = {},
		)
	}
}
