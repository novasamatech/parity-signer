package io.parity.signer.screens.keysetdetails.export

import android.content.res.Configuration
import androidx.compose.foundation.*
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.ExperimentalComposeUiApi
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalInspectionMode
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.res.pluralStringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.BottomSheetHeader
import io.parity.signer.components.base.NotificationFrameText
import io.parity.signer.components.base.SignerDivider
import io.parity.signer.components.qrcode.AnimatedQrKeysInfo
import io.parity.signer.components.qrcode.EmptyAnimatedQrKeysProvider
import io.parity.signer.components.sharedcomponents.KeyCard
import io.parity.signer.components.sharedcomponents.KeyCardModel
import io.parity.signer.domain.*
import io.parity.signer.ui.theme.*

@OptIn(ExperimentalComposeUiApi::class)
@Composable
fun KeySetDetailsExportResultBottomSheet(
	model: KeySetDetailsModel,
	selectedKeys: Set<String>,
	onClose: Callback,
) {
	//preconditions
	if (model.root == null) {
		//todo key details
		submitErrorState(
			"KeySetDetailsExport desont' work without root and " +
				"root is missing, returning and it's ugly user experience, should be " +
				"impossible to navigate here "
		)
		onClose()
	} else {
		Column(Modifier.background(MaterialTheme.colors.backgroundTertiary)) {
			BottomSheetHeader(
				title = pluralStringResource(
					id = R.plurals.key_export_title,
					count = selectedKeys.size,
					selectedKeys.size,
				),
				onCloseClicked = onClose
			)
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
				if (LocalInspectionMode.current) {
					AnimatedQrKeysInfo(
						input = Unit,
						provider = EmptyAnimatedQrKeysProvider(),
						modifier = Modifier.padding(8.dp)
					)
				} else {
					AnimatedQrKeysInfo(
						input = KeySetDetailsExportService.GetQrCodesListRequest(
							seedName = model.root.seedName,
							keys = model.keysAndNetwork.map { it.key }
								.filter { selectedKeys.contains(it.addressKey) }),
						provider = KeySetDetailsExportService(),
						modifier = Modifier.padding(8.dp)
					)
				}
				NotificationFrameText(messageRes = R.string.key_set_export_description_content)
				val seedList = selectedKeys.toList()
				for (i in 0..seedList.lastIndex) {
					val seed = seedList[i]
					val keyModel = model.keysAndNetwork
						.first { it.key.addressKey == seed }
					KeyCard(
						KeyCardModel.fromKeyModel(
							keyModel.key,
							keyModel.network
						),
					)
					if (i != seedList.lastIndex) {
						SignerDivider()
					}
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
			style = SignerTypeface.BodyM,
		)
		Text(
			text = " · ",
			color = MaterialTheme.colors.textTertiary,
			style = SignerTypeface.BodyM,
		)
		Text(
			text = pluralStringResource(
				id = R.plurals.key_sets_item_derived_subtitle,
				count = seed.derivedKeysCount.toInt(),
				seed.derivedKeysCount.toInt(),
			),
			color = MaterialTheme.colors.textTertiary,
			style = SignerTypeface.BodyM,
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
	val selected = setOf(
		model.keysAndNetwork[0].key.addressKey,
		model.keysAndNetwork[1].key.addressKey,
	)
	SignerNewTheme {
		Box(modifier = Modifier.size(350.dp, 700.dp)) {
			KeySetDetailsExportResultBottomSheet(model, selected, {})
		}
	}
}
