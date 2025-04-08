package io.parity.signer.screens.scan.bananasplitcreate.show

import android.content.res.Configuration
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalInspectionMode
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.NotificationFrameText
import io.parity.signer.components.base.ScreenHeaderClose
import io.parity.signer.components.qrcode.AnimatedQrKeysInfo
import io.parity.signer.components.qrcode.EmptyAnimatedQrKeysProvider
import io.parity.signer.components.qrcode.EmptyQrCodeProvider
import io.parity.signer.domain.Callback
import io.parity.signer.domain.getData
import io.parity.signer.screens.keysetdetails.export.KeySetDetailsExportService
import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.fill6
import io.parity.signer.uniffi.QrData


@Composable
fun BananaSplitExportScreen(
	qrCodes: List<QrData>,
	onMenu: Callback,
	onClose: Callback,
	modifier: Modifier = Modifier,
) {
	Column(modifier.fillMaxHeight(1f)) {
		ScreenHeaderClose(title = "", onClose = onClose, onMenu = onMenu)
		Column(
			modifier = Modifier
				.verticalScroll(rememberScrollState())
				.weight(weight = 1f, fill = false)
				.padding(start = 16.dp, end = 16.dp, bottom = 48.dp, top = 48.dp)
		) {
			if (LocalInspectionMode.current) {
				AnimatedQrKeysInfo(
					input = Unit,
					provider = EmptyAnimatedQrKeysProvider(),
					modifier = Modifier.padding(8.dp)
				)
			} else {
				AnimatedQrKeysInfo(
					input = qrCodes.map { it.getData() },
					provider = EmptyQrCodeProvider(),
					modifier = Modifier.padding(8.dp)
				)
			}
			NotificationFrameText(message = stringResource(R.string.create_bs_export_notification_text))
		}
	}
}


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
private fun PreviewBananaSplitExportScreen() {
	SignerNewTheme {
		BananaSplitExportScreen(
			qrCodes = listOf(
				QrData.Regular(PreviewData.exampleQRData),
			),
			onClose = {},
			onMenu = {},
		)
	}
}
