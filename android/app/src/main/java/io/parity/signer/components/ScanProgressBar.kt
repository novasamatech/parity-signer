package io.parity.signer.components

import androidx.compose.foundation.Canvas
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.geometry.Size
import androidx.compose.ui.graphics.drawscope.Stroke
import androidx.compose.ui.unit.dp
import io.parity.signer.models.SignerDataModel
import io.parity.signer.models.resetScan
import io.parity.signer.ui.theme.Text400
import io.parity.signer.ui.theme.Text600

@Composable
fun ScanProgressBar(signerDataModel: SignerDataModel) {
	val progress = signerDataModel.progress.observeAsState()
	val frontColor = MaterialTheme.colors.onSecondary
	val captured = signerDataModel.captured.observeAsState()
	val total = signerDataModel.total.observeAsState()

	progress.value?.let {
		if (it != 0f) {
			Surface {
				Column {
					HeadingOverline(text = "PARSING MULTIPART DATA")
					Canvas(
						modifier = Modifier
							.height(24.dp)
							.fillMaxWidth()
					) {
						drawRect(
							frontColor,
							Offset.Zero.copy(x = 0.dp.toPx(), y = 8.dp.toPx()),
							Size(width = this.size.width, height = 8.dp.toPx()),
							style = Stroke()
						)
						drawRect(
							frontColor,
							Offset.Zero.copy(x = 0.dp.toPx(), y = 8.dp.toPx()),
							Size(
								width = this.size.width * it,
								height = 8.dp.toPx()
							)
						)
					}
					Text(
						"From " + captured.value.toString() + " / " + total.value.toString() + " captured frames",
						style = MaterialTheme.typography.subtitle1,
						color = MaterialTheme.colors.Text600
					)
					Text(
						"Please hold still",
						style = MaterialTheme.typography.subtitle2,
						color = MaterialTheme.colors.Text400
					)
					BigButton(
						text = "Start over",
						action = { signerDataModel.resetScan() })
				}
			}
		}
	}
}
