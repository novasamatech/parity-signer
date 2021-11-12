package io.parity.signer.components

import androidx.compose.foundation.Canvas
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.runtime.Composable
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.geometry.Size
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.PaintingStyle.Companion.Fill
import androidx.compose.ui.graphics.StrokeCap
import androidx.compose.ui.graphics.drawscope.DrawStyle
import androidx.compose.ui.graphics.drawscope.Stroke
import androidx.compose.ui.unit.dp
import io.parity.signer.models.SignerDataModel

@Composable
fun ScanProgressBar(signerDataModel: SignerDataModel) {
	val progress = signerDataModel.progress.observeAsState()

	progress.value?.let {
		Canvas(modifier = Modifier.height(16.dp).fillMaxWidth()) {
			drawLine(
				Color.Blue,
				Offset.Zero.copy(x = 4.dp.toPx(), y = 12.dp.toPx()),
				Offset.Zero.copy(x = this.size.width - 4.dp.toPx(), y = 12.dp.toPx()),
				8.dp.toPx(),
				StrokeCap.Round
			)
			drawLine(
				Color.Black,
				Offset.Zero.copy(x = 5.dp.toPx(), y = 13.dp.toPx()),
				Offset.Zero.copy(
					x = this.size.width - 5.dp.toPx(),
					y = 13.dp.toPx()
				),
				6.dp.toPx(),
				StrokeCap.Round
			)
			drawLine(
				Color.Blue,
				Offset.Zero.copy(x = 5.dp.toPx(), y = 13.dp.toPx()),
				Offset.Zero.copy(x = (this.size.width - 5.dp.toPx()) * it, y = 13.dp.toPx()),
				6.dp.toPx(),
				StrokeCap.Round
			)
		}
	}
}
