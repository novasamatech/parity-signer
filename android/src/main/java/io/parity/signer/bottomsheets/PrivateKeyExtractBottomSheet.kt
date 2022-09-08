package io.parity.signer.bottomsheets

import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.gestures.Orientation
import androidx.compose.foundation.gestures.draggable
import androidx.compose.foundation.gestures.rememberDraggableState
import androidx.compose.foundation.layout.*
import androidx.compose.material.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.unit.dp
import io.parity.signer.components.KeyCard
import io.parity.signer.components.NetworkCard
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.intoImageBitmap
import io.parity.signer.models.navigate
import io.parity.signer.ui.theme.Bg000
import io.parity.signer.ui.theme.Bg200
import io.parity.signer.ui.theme.modal
import io.parity.signer.uniffi.Action

@OptIn(ExperimentalMaterialApi::class)
@Composable
fun PrivateKeyExtractBottomSheet(model: PrivateKeyExtractModel,
																 signerDataModel: SignerDataModel) {

//	val bottomSheetScaffoldState = rememberBottomSheetScaffoldState(
//		bottomSheetState = BottomSheetState(BottomSheetValue.Collapsed)
//	)
//	val coroutineScope = rememberCoroutineScope()

	Column (
		Modifier.clickable { signerDataModel.navigate(Action.GO_BACK) }
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
//					NetworkCard(network = keyDetailsMulti.keyDetails.networkInfo)
				}
				Image(
					model.qrImage.intoImageBitmap(),
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

class PrivateKeyExtractModel(var qrImage: List<UByte>, )
