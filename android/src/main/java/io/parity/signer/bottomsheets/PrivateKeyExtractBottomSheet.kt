package io.parity.signer.bottomsheets

import android.content.res.Configuration.UI_MODE_NIGHT_NO
import android.content.res.Configuration.UI_MODE_NIGHT_YES
import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.asImageBitmap
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalInspectionMode
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.core.graphics.drawable.toBitmap
import io.parity.signer.components.NetworkCard
import io.parity.signer.components.NetworkCardModel
import io.parity.signer.models.EmptyNavigator
import io.parity.signer.models.Navigator
import io.parity.signer.models.intoImageBitmap
import io.parity.signer.ui.theme.Bg000
import io.parity.signer.ui.theme.Bg200
import io.parity.signer.ui.theme.modal
import io.parity.signer.uniffi.Action
import io.parity.signer.R

@Composable
fun PrivateKeyExtractBottomSheet(
	model: PrivateKeyExtractModel,
	navigator: Navigator,
) {

//	val bottomSheetScaffoldState = rememberBottomSheetScaffoldState(
//		bottomSheetState = BottomSheetState(BottomSheetValue.Collapsed)
//	)
//	val coroutineScope = rememberCoroutineScope()

	Column(
		Modifier.clickable { navigator.navigate(Action.GO_BACK) }
	) {
		Spacer(Modifier.weight(1f))
		Surface(
			color = MaterialTheme.colors.Bg000,
			shape = MaterialTheme.shapes.modal
		) {
			Column(
				modifier = Modifier
					.fillMaxWidth()
			) {
				Row(
					Modifier
						.padding(top = 3.dp, start = 12.dp, end = 12.dp)
						.background(
							MaterialTheme.colors.Bg200
						)
						.fillMaxWidth()
				) {
//					KeyCard(identity = keyDetailsMulti.keyDetails.address)
					Text("Export Private Key")
				}
				Row(
					Modifier.padding(top = 3.dp, start = 12.dp, end = 12.dp)
				) {
					NetworkCard(network = model.network)
				}
				Image(
					if (LocalInspectionMode.current) {
						LocalContext.current.getDrawable(R.drawable.icon)!!.toBitmap()
							.asImageBitmap()
					} else {
						model.qrImage.intoImageBitmap()
					},
					contentDescription = "QR with address to scan",
					contentScale = ContentScale.FillWidth,
					modifier = Modifier
						.fillMaxWidth(1f)
						.padding(12.dp)
				)
//				Text(
//					"Key " + keyDetailsMulti.currentNumber + " out of " + keyDetailsMulti.outOf
//				)
			}

		}
	}
}

class PrivateKeyExtractModel(
	val qrImage: List<UByte>,
	val network: NetworkCardModel
) {
	companion object {
		fun createMock() = PrivateKeyExtractModel(
			0u.rangeTo(200u).map { it.toUByte() }.toList(),
			NetworkCardModel("NetworkTitle", "NetworkLogo")
		)
	}
}

@Preview(
	name = "day",
	group = "themes",
	uiMode = UI_MODE_NIGHT_NO
)
@Composable
private fun PrivateKeyExtractBottomSheetPreview() {
	PrivateKeyExtractBottomSheet(
		model = PrivateKeyExtractModel.createMock(),
		EmptyNavigator()
	)
}

@Preview(
	name = "dark theme",
	group = "themes",
	uiMode = UI_MODE_NIGHT_YES
)
@Composable
private fun PrivateKeyExtractBottomSheetPreviewNight() {
	PrivateKeyExtractBottomSheet(
		model = PrivateKeyExtractModel.createMock(),
		EmptyNavigator()
	)
}
