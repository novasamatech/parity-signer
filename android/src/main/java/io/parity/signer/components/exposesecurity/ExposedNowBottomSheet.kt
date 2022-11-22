package io.parity.signer.components.exposesecurity

import android.content.res.Configuration.UI_MODE_NIGHT_NO
import android.content.res.Configuration.UI_MODE_NIGHT_YES
import androidx.compose.foundation.*
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.ColorFilter
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.screens.keydetails.exportprivatekey.PrivateKeyExportModel.Companion.SHOW_PRIVATE_KEY_TIMEOUT
import io.parity.signer.components.NetworkCardModel
import io.parity.signer.components.sharedcomponents.CircularCountDownTimer
import io.parity.signer.components.sharedcomponents.KeyCard
import io.parity.signer.components.base.BottomSheetHeader
import io.parity.signer.components.base.PrimaryButtonBottomSheet
import io.parity.signer.models.EmptyNavigator
import io.parity.signer.models.KeyCardModel
import io.parity.signer.models.Navigator
import io.parity.signer.models.intoImageBitmap
import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.ui.theme.*
import io.parity.signer.uniffi.Action

@Composable
fun ExposedNowBottomSheet(
	navigator: Navigator,
) {
	val sidePadding = 24.dp
	Column(
		modifier = Modifier
			.fillMaxWidth(),
		horizontalAlignment = Alignment.CenterHorizontally,
	) {

		Image(
			painter = painterResource(id = R.drawable.ic_shield_exposed_32),
			contentDescription = stringResource(R.string.description_shield_exposed_icon),
			colorFilter = ColorFilter.tint(MaterialTheme.colors.red400),
			modifier = Modifier.size(80.dp),
		)

		Text(
			text = "Title",
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.TitleL,
			modifier = Modifier.fillMaxWidth(1f).padding(horizontal = 24.dp),
		)

		Text(
			text = "message",
			color = MaterialTheme.colors.textSecondary,
			style = SignerTypeface.BodyL,
			modifier = Modifier.fillMaxWidth(1f).padding(horizontal = 24.dp),

			)

		PrimaryButtonBottomSheet(
			label = stringResource(R.string.general_got_it),
			modifier = Modifier.padding(horizontal = 32.dp),
			isNeutral = true,
		) {
			navigator.navigate(Action.GO_BACK)
		}
	}
}


@Preview(
	name = "light", group = "themes", uiMode = UI_MODE_NIGHT_NO,
	showBackground = true, backgroundColor = 0xFFFFFFFF,
)
@Preview(
	name = "dark", group = "themes", uiMode = UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun PreviewExposedNowBottomSheet() {
	SignerNewTheme {
		ExposedNowBottomSheet(
			EmptyNavigator()
		)
	}
}

