package io.parity.signer.screens.settings.networks.signspecs.view

import android.content.res.Configuration
import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
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
import io.parity.signer.components.NetworkCard
import io.parity.signer.components.base.BottomSheetHeader
import io.parity.signer.components.networkicon.IdentIconImage
import io.parity.signer.components.sharedcomponents.KeyCardSignature
import io.parity.signer.components.toNetworkCardModel
import io.parity.signer.domain.Callback
import io.parity.signer.domain.intoImageBitmap
import io.parity.signer.screens.settings.networks.signspecs.SignSpecsResultModel
import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.uniffi.MscContent

@Composable
internal fun SignSpecsResultBottomSheet(
	model: SignSpecsResultModel,
	onBack: Callback,
) {
	Column(
		modifier = Modifier
			.fillMaxWidth()
			.verticalScroll(rememberScrollState())
	) {
		BottomSheetHeader(
			title = stringResource(R.string.sign_specs_result_title),
			onCloseClicked = onBack
		)
		Box(
			modifier = Modifier
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
					SufficientCryptoReadyViewModel.getQrCodeBitmapFromQrCodeData(
						model.sufficientSignature
					)!!
				}
			}
			Image(
				bitmap = qrImage.intoImageBitmap(),
				contentDescription = "Signed update",
				contentScale = ContentScale.Fit,
				modifier = Modifier.size(264.dp)
			)
		}

		KeyCardSignature(
			model.key,
			modifier = Modifier
				.padding(top = 16.dp, bottom = 24.dp)
				.padding(horizontal = 24.dp)
		)
		Box(
			modifier = Modifier
				.padding(bottom = 24.dp)
				.padding(horizontal = 24.dp)
		) {
			when (val c = model.content) {
				is MscContent.AddSpecs -> Column {
					Text(
						text = stringResource(R.string.sign_spes_result_specs_lable),
						color = MaterialTheme.colors.primary,
						style = SignerTypeface.LabelM,
					)
					NetworkCard(c.f.toNetworkCardModel())
				}

				is MscContent.LoadMetadata -> Text(
					stringResource(
						R.string.sign_specs_load_metadata_label,
						c.name,
						c.version
					),
					color = MaterialTheme.colors.primary,
					style = SignerTypeface.LabelM,
				)

				is MscContent.LoadTypes -> Column {
					Text(
						stringResource(R.string.sign_specs_load_types_label, c.types),
						color = MaterialTheme.colors.primary,
						style = SignerTypeface.LabelM,
					)
					IdentIconImage(identicon = c.pic)
				}
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
private fun PreviewSignSpecsResultBottomSheet() {
	SignerNewTheme {
		SignSpecsResultBottomSheet(
			model = SignSpecsResultModel.createStub(),
			onBack = {},
		)
	}
}
