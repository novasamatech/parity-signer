package io.parity.signer.components

import androidx.compose.foundation.Canvas
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.MaterialTheme
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
	val frontColor = MaterialTheme.colors.onSecondary
	val backgroundColor = MaterialTheme.colors.background

	progress.value?.let {
		Canvas(modifier = Modifier
			.height(24.dp)
			.fillMaxWidth()) {
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
	}
}
