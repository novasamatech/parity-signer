package io.parity.signer.components2

import android.widget.Toast
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.padding
import androidx.compose.material.CircularProgressIndicator
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.models.submitErrorState
import io.parity.signer.ui.theme.SignerNewTheme

@Composable
fun CountDown(timeoutSeconds: Int, text: String, onTimeOutAction: () -> Unit) {
	if (timeoutSeconds <= 0) {
		onTimeOutAction()
		submitErrorState("timer started with wrong value $timeoutSeconds")
	}
	val timeLeft = timeoutSeconds
	Row(
		modifier = Modifier.padding(24.dp),
		verticalAlignment = Alignment.CenterVertically
	) {
		Text(text = text)
		val progress: Float = timeoutSeconds.toFloat() / timeLeft
		Box(contentAlignment = Alignment.Center) {
			CircularProgressIndicator(progress = progress)
			Text(
				text = timeLeft.toString(),
				color = MaterialTheme.colors.primary,
				style = MaterialTheme.typography.h3,
			)
		}
	}
}


@Preview
@Composable
private fun PreviewCountDown() {
	SignerNewTheme() {
		val context = LocalContext.current
		CountDown(timeoutSeconds = 30, text = "Time is going out") {
			Toast.makeText(context, "time is up", Toast.LENGTH_LONG).show()
		}
	}
}
