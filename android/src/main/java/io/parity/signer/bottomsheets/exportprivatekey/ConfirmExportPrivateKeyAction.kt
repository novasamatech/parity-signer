package io.parity.signer.bottomsheets.exportprivatekey

import android.content.res.Configuration
import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Scaffold
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.painter.Painter
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.core.content.res.ResourcesCompat
import io.parity.signer.R
import io.parity.signer.components2.CircularCountDownTimer
import io.parity.signer.components2.KeyCard
import io.parity.signer.components2.base.BottomSheetHeader
import io.parity.signer.models.EmptyNavigator
import io.parity.signer.models.Navigator
import io.parity.signer.models.intoImageBitmap
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.appliedStroke
import io.parity.signer.ui.theme.fill6


@Composable
fun ConfirmExportPrivateKeyAction(
	navigator: Navigator,
) {
	val sidePadding = 24.dp
	Column(
		modifier = Modifier
			.fillMaxWidth()
			.padding(start = sidePadding, end = sidePadding),
		horizontalAlignment = Alignment.CenterHorizontally,
	) {
		val drawable = painterResource(R.drawable.private_key)
		Icon(drawable, null)
		Text(text = "Export Private Key")
		Text(text = "A private key can be used to Sign transactions.This key will be marked as a hot key after export.")
		Box() {
			Text(text = "Export Private Key confirm")
		}
		Box() {
			Text(text = "Cancel")
		}

	}
}


@Preview(
	name = "day", group = "themes", uiMode = Configuration.UI_MODE_NIGHT_NO,
	showBackground = true,backgroundColor = 0xFFFFFFFF,
)
@Preview(
	name = "dark theme", group = "themes", uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000
)
@Composable
private fun PreviewPrivateKeyExportBottomSheet() {
	SignerNewTheme {
		ConfirmExportPrivateKeyAction(
			EmptyNavigator(),
		)
	}
}
