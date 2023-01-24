package io.parity.signer.screens.keyderivation.help

import android.content.res.Configuration
import androidx.compose.foundation.*
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.outlined.Info
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.ExperimentalComposeUiApi
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalInspectionMode
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.pluralStringResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.BottomSheetHeader
import io.parity.signer.components.base.SecondaryButtonWide
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.components.qrcode.AnimatedQrKeysInfo
import io.parity.signer.components.qrcode.EmptyAnimatedQrKeysProvider
import io.parity.signer.models.Callback
import io.parity.signer.models.KeySetModel
import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.ui.theme.*

@Composable
fun DerivationMethodsHelpBottomSheet(
	onClose: Callback,
) {
	Column(Modifier.background(MaterialTheme.colors.backgroundTertiary)) {
		BottomSheetHeader(
			title = stringResource(R.string.derivation_help_methods_title),
			onCloseClicked = onClose,
		)
		//scrollable part
		Column(
			modifier = Modifier
				.verticalScroll(rememberScrollState())
				.padding(horizontal = 24.dp)
		) {
			Text(
				text = stringResource(R.string.derivation_help_path_header1),
				color = MaterialTheme.colors.primary,
				style = SignerTypeface.TitleS,
				modifier = Modifier.padding(vertical = 12.dp),
			)
			Text(
				text = stringResource(R.string.derivation_help_path_message1),
				color = MaterialTheme.colors.textSecondary,
				style = SignerTypeface.BodyM,
			)
			Text(
				text = stringResource(R.string.derivation_help_path_header2),
				color = MaterialTheme.colors.primary,
				style = SignerTypeface.TitleS,
				modifier = Modifier.padding(vertical = 12.dp),
			)
			Text(
				text = stringResource(R.string.derivation_help_path_message2),
				color = MaterialTheme.colors.textSecondary,
				style = SignerTypeface.BodyM,
			)
			Text(
				text = stringResource(R.string.derivation_help_path_header3),
				color = MaterialTheme.colors.primary,
				style = SignerTypeface.TitleS,
				modifier = Modifier.padding(vertical = 12.dp),
			)
			Text(
				text = stringResource(R.string.derivation_help_path_message3),
				color = MaterialTheme.colors.textSecondary,
				style = SignerTypeface.BodyM,
			)
			Spacer(modifier = Modifier.padding(bottom = 24.dp))
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
private fun PreviewDerivationMethodsHelpBottomSheet() {
	SignerNewTheme {
		DerivationMethodsHelpBottomSheet({})
	}
}
