//package io.parity.signer.bottomSheets
//
//import androidx.compose.foundation.layout.*
//import androidx.compose.material.*
//import androidx.compose.runtime.*
//import androidx.compose.ui.Modifier
//import androidx.compose.ui.unit.dp
//import io.parity.signer.alerts.AndroidCalledConfirm
//import io.parity.signer.components.BigButton
//import io.parity.signer.components.HeaderBar
//import io.parity.signer.models.pushButton
//import io.parity.signer.ui.theme.Bg000
//import io.parity.signer.ui.theme.modal
//import io.parity.signer.uniffi.Action
//
//@OptIn(ExperimentalMaterialApi::class)
//@Composable
//fun PrivateKeyExtractBottomSheet() {
//
//	val bottomSheetScaffoldState = rememberBottomSheetScaffoldState(
//		bottomSheetState = BottomSheetState(BottomSheetValue.Collapsed)
//	)
//	val coroutineScope = rememberCoroutineScope()
////	BottomSheetScaffold(
////		scaffoldState = bottomSheetScaffoldState,
////		sheetContent = {
////			Box(
////				Modifier
////					.fillMaxWidth()
////					.height(200.dp)
////			) {
////				Text(text = "Hello from sheet")
////			}
////		}, sheetPeekHeight = 0.dp
////	)
//
//	Column (
////		Modifier.clickable { close() } //todo dmitry
//	) {
//		Spacer(Modifier.weight(1f))
//		Surface(
//			color = MaterialTheme.colors.Bg000,
//			shape = MaterialTheme.shapes.modal
//		) {
//			Column(
//				modifier = Modifier.padding(20.dp)
//			) {
//				HeaderBar(line1 = "KEY MENU", line2 = "Select action")
//				BigButton(
//					text = "Export Private Key",
//					isShaded = true,
//					isDangerous = false,
//					action = {
//						confirmExport = true
//					}
//				)
//				BigButton(
//					text = "Forget this key forever",
//					isShaded = true,
//					isDangerous = true,
//					action = {
//						confirmForget = true
//					}
//				)
//			}
//		}
//	}
//	AndroidCalledConfirm(
//		show = confirmForget,
//		header = "Forget this key?",
//		text = "This key will be removed for this network. Are you sure?",
//		back = { confirmForget = false },
//		forward = { signerDataModel.pushButton(Action.REMOVE_KEY) },
//		backText = "Cancel",
//		forwardText = "Remove key"
//	)
//	AndroidCalledConfirm(
//		show = confirmExport,
//		header = "Export Private Key",
//		text = "A private key can be used to sign transactions. This key will be marked as a hot key after export.",
//		back = { confirmExport = false },
//		forward = { signerDataModel.pushButton(Action.REMOVE_KEY) }, //TODO dmitry
//		backText = "Cancel",
//		forwardText = "Export Private Key"
//	)
//}
