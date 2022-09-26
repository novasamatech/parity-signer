package io.parity.signer.bottomsheets

import android.content.res.Configuration.UI_MODE_NIGHT_NO
import android.content.res.Configuration.UI_MODE_NIGHT_YES
import androidx.compose.foundation.*
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.bottomsheets.PrivateKeyExportModel.Companion.SHOW_PRIVATE_KEY_TIMEOUT
import io.parity.signer.components.NetworkCardModel
import io.parity.signer.components2.CircularCountDownTimer
import io.parity.signer.components2.KeyCard
import io.parity.signer.components2.KeyCardModel
import io.parity.signer.components2.base.BottomSheetHeader
import io.parity.signer.models.EmptyNavigator
import io.parity.signer.models.Navigator
import io.parity.signer.models.intoImageBitmap
import io.parity.signer.ui.helpers.PreviewData
import io.parity.signer.ui.theme.*

@Composable
fun PrivateKeyExportBottomSheet(
	model: PrivateKeyExportModel,
	navigator: Navigator,
) {
	Column(
		modifier = Modifier
            .clickable { navigator.backAction() }
            .fillMaxWidth()
	) {
		Spacer(Modifier.weight(1f))
		Surface(
			color = MaterialTheme.colors.backgroundSecondary,
			shape = MaterialTheme.shapes.modal
		) {
			val sidePadding = 24.dp
			Column(
				modifier = Modifier
                    .fillMaxWidth()
                    .padding(start = sidePadding, end = sidePadding),
				horizontalAlignment = Alignment.CenterHorizontally,
			) {
				BottomSheetHeader(stringResource(R.string.export_private_key_title)) {
					navigator.backAction()
				}

				val qrRounding = 16.dp
				val plateShape =
					RoundedCornerShape(qrRounding, qrRounding, qrRounding, qrRounding)
				Column(
					modifier = Modifier
                        .clip(plateShape)
                        .border(
                            BorderStroke(1.dp, MaterialTheme.colors.appliedStroke),
                            plateShape
                        )
                        .background(MaterialTheme.colors.fill6, plateShape)
				) {
					Box(
						modifier = Modifier
                            .fillMaxWidth(1f)
                            .aspectRatio(1.1f)
                            .background(
                                Color.White,
                                RoundedCornerShape(qrRounding)
                            ),
						contentAlignment = Alignment.Center,
					) {
						Image(
							bitmap = model.qrImage.intoImageBitmap(),
							contentDescription = stringResource(R.string.qr_with_address_to_scan_description),
							contentScale = ContentScale.Fit,
							modifier = Modifier.size(264.dp)
						)
					}
					KeyCard(model.keyCard)
				}
				//autohide component
				val timerText = stringResource(R.string.export_private_key_timer_label)
				CircularCountDownTimer(
					SHOW_PRIVATE_KEY_TIMEOUT,
					timerText
				) { navigator.backAction() }
			}
		}
	}
}

class PrivateKeyExportModel(
	val qrImage: List<UByte>,
	val keyCard: KeyCardModel,
	val network: NetworkCardModel,
) {
	companion object {
		const val SHOW_PRIVATE_KEY_TIMEOUT = 60 //seconds

		fun createMock(): PrivateKeyExportModel = PrivateKeyExportModel(
			qrImage = PreviewData.exampleQRCode,
			keyCard = KeyCardModel.createStub(),
			network = NetworkCardModel("Polkadot", "NetworkLogo")
		)
	}
}

@Preview(
	name = "day",
	group = "themes",
	uiMode = UI_MODE_NIGHT_NO,
//	showBackground = true,
//	backgroundColor = 0xFFFFFFFF,
)
@Preview(
	name = "dark theme", group = "themes", uiMode = UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000
)
@Composable
private fun PreviewPrivateKeyExportBottomSheet() {
	SignerNewTheme {
		PrivateKeyExportBottomSheet(
			model = PrivateKeyExportModel.createMock(),
			EmptyNavigator()
		)
	}
}

