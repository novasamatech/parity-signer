package io.parity.signer.screens.keysets.create

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
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.components.qrcode.AnimatedQrKeysInfo
import io.parity.signer.components.qrcode.EmptyAnimatedQrKeysProvider
import io.parity.signer.components.sharedcomponents.KeyCard
import io.parity.signer.components.sharedcomponents.KeySeedCard
import io.parity.signer.models.Callback
import io.parity.signer.models.KeyCardModel
import io.parity.signer.models.KeySetDetailsModel
import io.parity.signer.models.KeySetModel
import io.parity.signer.ui.theme.*

@Composable
internal fun NewKeySetBackupBottomSheet(
	model: NewSeedBackupModel,
	onClose: Callback,
) {
	Column(Modifier.background(MaterialTheme.colors.backgroundTertiary))
	{

		val plateShape =
			RoundedCornerShape(dimensionResource(id = R.dimen.qrShapeCornerRadius))
		//scrollable part
		Column(
			modifier = Modifier
				.verticalScroll(rememberScrollState())
				.weight(weight = 1f, fill = false)
				.padding(start = 16.dp, end = 16.dp, bottom = 16.dp)
				.background(MaterialTheme.colors.fill6, plateShape)
		) {

			val innerShape =
				RoundedCornerShape(dimensionResource(id = R.dimen.innerFramesCornerRadius))
			Row(
				modifier = Modifier
					.padding(8.dp)
					.border(
						BorderStroke(1.dp, MaterialTheme.colors.appliedStroke),
						innerShape
					)
					.background(MaterialTheme.colors.fill6, innerShape)

			) {
				Text(
					text = stringResource(R.string.key_set_export_description_content),
					color = MaterialTheme.colors.textTertiary,
					style = SignerTypeface.CaptionM,
					modifier = Modifier
						.weight(1f)
						.padding(start = 16.dp, top = 16.dp, bottom = 16.dp)
				)
				Icon(
					imageVector = Icons.Outlined.Info,
					contentDescription = null,
					tint = MaterialTheme.colors.pink300,
					modifier = Modifier
						.align(Alignment.CenterVertically)
						.padding(start = 18.dp, end = 18.dp)
				)
			}

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
private fun PreviewNewKeySetBackupBottomSheet() {
	val model = NewSeedBackupModel.createStub()
	SignerNewTheme {
			NewKeySetBackupBottomSheet(model, {})
	}
}
