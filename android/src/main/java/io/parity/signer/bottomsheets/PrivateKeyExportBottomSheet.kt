package io.parity.signer.bottomsheets

import android.content.res.Configuration.UI_MODE_NIGHT_NO
import android.content.res.Configuration.UI_MODE_NIGHT_YES
import androidx.compose.foundation.*
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.asImageBitmap
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalInspectionMode
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.core.graphics.drawable.toBitmap
import io.parity.signer.R
import io.parity.signer.components.NetworkCardModel
import io.parity.signer.components2.KeyCard
import io.parity.signer.components2.KeyCardModel
import io.parity.signer.models.EmptyNavigator
import io.parity.signer.models.Navigator
import io.parity.signer.models.intoImageBitmap
import io.parity.signer.ui.theme.*
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.Address

@Composable
fun PrivateKeyExportBottomSheet(
	model: PrivateKeyExportModel,
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
						.fillMaxWidth()
				) {
					Text(text = "Export Private Key")
				}
				val plateShape = RoundedCornerShape(16.dp, 16.dp, 16.dp, 16.dp)
				Column(
					modifier = Modifier
						.padding(top = 3.dp, start = 12.dp, end = 12.dp)
						.clip(plateShape)
						.border(BorderStroke(1.dp, MaterialTheme.colors.fill12), plateShape)
						.background(MaterialTheme.colors.fill12, plateShape)
				) {
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
							.clip(RoundedCornerShape(16.dp))
					)
					KeyCard(KeyCardModel.fromAddress(model.address, model.network.networkTitle))
					Spacer(modifier = Modifier.padding(bottom = 4.dp))
				}
				Spacer(modifier = Modifier.padding(bottom = 24.dp))
			}
		}
	}
}

class PrivateKeyExportModel(
	val qrImage: List<UByte>,
	val address: Address,
	val network: NetworkCardModel,
) {
	companion object {
		fun createMock(): PrivateKeyExportModel = PrivateKeyExportModel(
			0u.rangeTo(200u).map { it.toUByte() }.toList(),
			Address(
				"base58", "path", true, listOf(0u, 1u),
				"seedName", false, true
			),
			NetworkCardModel("NetworkTitle", "NetworkLogo")
		)
	}
}

@Preview(name = "day", group = "themes", uiMode = UI_MODE_NIGHT_NO)
@Preview(name = "dark theme", group = "themes", uiMode = UI_MODE_NIGHT_YES, showBackground = true)
@Composable
private fun PreviewPrivateKeyExportBottomSheet() {
	SignerNewTheme {
		PrivateKeyExportBottomSheet(
			model = PrivateKeyExportModel.createMock(),
			EmptyNavigator()
		)
	}
}

