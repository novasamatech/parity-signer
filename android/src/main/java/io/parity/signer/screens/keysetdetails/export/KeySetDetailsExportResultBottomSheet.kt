package io.parity.signer.screens.keysetdetails.export

import android.content.res.Configuration
import androidx.compose.foundation.*
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.Divider
import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.outlined.Info
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.ExperimentalComposeUiApi
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.pluralStringResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.BottomSheetHeader
import io.parity.signer.models.Callback
import io.parity.signer.models.KeySetDetailsModel
import io.parity.signer.models.KeySetModel
import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.ui.theme.*

@OptIn(ExperimentalComposeUiApi::class)
@Composable
fun KeySetDetailsExportResultBottomSheet(
	model: KeySetDetailsModel,
	selectedKeys: Set<String>,
	onClose: Callback,
) {
	//todo dmitry migrate to keys and reuse qr code logic
	Column(Modifier.background(MaterialTheme.colors.backgroundTertiary)) {
		BottomSheetHeader(
			header = pluralStringResource(
				id = R.plurals.key_export_title,
				count = selectedKeys.size,
				selectedKeys.size,
			),
			onCloseClicked = onClose
		)
		val qrRounding = dimensionResource(id = R.dimen.qrShapeCornerRadius)
		val plateShape =
			RoundedCornerShape(qrRounding, qrRounding, qrRounding, qrRounding)
		//scrollable part
		Column(
			modifier = Modifier
				.verticalScroll(rememberScrollState())
				.weight(weight = 1f, fill = false)
				.padding(start = 16.dp, end = 16.dp, bottom = 16.dp)
				.background(MaterialTheme.colors.fill6, plateShape)
		) {

//			todo dmitry make it generic
//			AnimatedQrSeedInfo(seeds, Modifier.padding(8.dp))

			val innerRounding =
				dimensionResource(id = R.dimen.innerFramesCornerRadius)
			val innerShape =
				RoundedCornerShape(
					innerRounding,
					innerRounding,
					innerRounding,
					innerRounding
				)
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
					style = TypefaceNew.CaptionM,
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
				//todo dmitry add root key view
					Divider(
						color = MaterialTheme.colors.appliedSeparator,
						thickness = 1.dp,
						startIndent = 16.dp,
					)
			val seedList = selectedKeys.toList()
			for (i in 0..seedList.lastIndex) {
				val seed = seedList[i]
//				KeySetItemInExport(seed) todo dmitry key item
				if (i != seedList.lastIndex) {
					Divider(
						color = MaterialTheme.colors.appliedSeparator,
						thickness = 1.dp,
						startIndent = 16.dp,
					)
				}
			}
		}
	}
}

@Composable
@OptIn(ExperimentalComposeUiApi::class)
private fun KeySetItemInExport(seed: KeySetModel) {
	Row(Modifier.padding(16.dp, top = 12.dp, bottom = 12.dp)) {
		Text(
			text = seed.seedName,
			color = MaterialTheme.colors.primary,
			style = TypefaceNew.BodyM,
		)
		Text(
			text = " · ",
			color = MaterialTheme.colors.textTertiary,
			style = TypefaceNew.BodyM,
		)
		Text(
			text = pluralStringResource(
				id = R.plurals.key_sets_item_derived_subtitle,
				count = seed.derivedKeysCount.toInt(),
				seed.derivedKeysCount.toInt(),
			),
			color = MaterialTheme.colors.textTertiary,
			style = TypefaceNew.BodyM,
		)
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
private fun PreviewKeySetDetailsExportResultBottomSheet() {
val model = KeySetDetailsModel.createStub()
	val selected = setOf(model.keys[2].addressKey)
	SignerNewTheme {
		Box(modifier = Modifier.size(350.dp, 700.dp)) {
			KeySetDetailsExportResultBottomSheet(model, selected, {})
		}
	}
}
