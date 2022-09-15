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
import androidx.compose.ui.res.stringResource
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
import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.ui.theme.Bg000
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.fill12
import io.parity.signer.ui.theme.modal
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.Address

@Composable
fun PrivateKeyExportBottomSheet(
	model: PrivateKeyExportModel,
	navigator: Navigator,
) {
	Column(
		modifier = Modifier
			.clickable { navigator.navigate(Action.GO_BACK) }
			.fillMaxWidth()
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
					modifier = Modifier
						.padding(top = 3.dp, start = 12.dp, end = 12.dp)
						.fillMaxWidth()
				) {
					Text(
						text = stringResource(R.string.export_private_key_title),
						color = MaterialTheme.colors.primary
					)
				}

				val qrRounding = 16.dp
				val plateShape = RoundedCornerShape(qrRounding, qrRounding, qrRounding, qrRounding)
				Column(
					modifier = Modifier
						.padding(top = 3.dp, start = 12.dp, end = 12.dp)
						.clip(plateShape)
						.border(
							BorderStroke(1.dp, MaterialTheme.colors.fill12),
							plateShape
						)
						.background(MaterialTheme.colors.fill12, plateShape)
				) {
					Image(
						bitmap = model.qrImage.intoImageBitmap(),
						contentDescription = stringResource(R.string.qr_with_address_to_scan_description),
						contentScale = ContentScale.FillWidth,
						modifier = Modifier
							.fillMaxWidth(1f)
							.clip(RoundedCornerShape(qrRounding))
					)
					KeyCard(
						KeyCardModel.fromAddress(
							model.address,
							model.network.networkTitle
						)
					)
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
			qrImage = PreviewData.exampleQRCode,
			address = Address(
				base58 = "5F3sa2TJAWMqDhXG6jhV4N8ko9SxwGy8TpaNS1repo5EYjQX",
				path = "//polkadot//path",
				hasPwd = false,
				identicon = PreviewData.exampleIdenticon,
				seedName = "seedName",
				multiselect = false,
				secretExposed = true
			),
			network = NetworkCardModel("NetworkTitle", "NetworkLogo")
		)
	}
}

@Preview(name = "day", group = "themes", uiMode = UI_MODE_NIGHT_NO, showBackground = true,)
@Preview(name = "dark theme",	group = "themes",	uiMode = UI_MODE_NIGHT_YES,)
@Composable
private fun PreviewPrivateKeyExportBottomSheet() {
	SignerNewTheme {
		PrivateKeyExportBottomSheet(
			model = PrivateKeyExportModel.createMock(),
			EmptyNavigator()
		)
	}
}

