package io.parity.signer.components.sharedcomponents

import android.widget.Toast
import androidx.compose.animation.ExperimentalAnimationApi
import androidx.compose.animation.core.animateFloatAsState
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.padding
import androidx.compose.material.CircularProgressIndicator
import androidx.compose.material.MaterialTheme
import androidx.compose.material.ProgressIndicatorDefaults
import androidx.compose.material.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.models.submitErrorState
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.TypefaceNew
import io.parity.signer.ui.theme.pink300
import io.parity.signer.ui.theme.textSecondary
import kotlinx.coroutines.delay
import kotlin.time.ExperimentalTime
import kotlin.time.seconds

@OptIn(ExperimentalTime::class, ExperimentalAnimationApi::class)
@Composable
fun CircularCountDownTimer(
	timeoutSeconds: Int,
	text: String,
	onTimeOutAction: () -> Unit
) {
	if (timeoutSeconds <= 0) {
		onTimeOutAction()
		submitErrorState("timer started with wrong value $timeoutSeconds")
	}

	var timeLeft by remember { mutableStateOf<Int>(timeoutSeconds) }
	Row(
		modifier = Modifier.padding(24.dp),
		verticalAlignment = Alignment.CenterVertically
	) {
		Text(
			text = text,
			color = MaterialTheme.colors.textSecondary,
			style = TypefaceNew.BodyL,
		)
		Box(
			modifier = Modifier.padding(start = 8.dp),
			contentAlignment = Alignment.Center
		) {
			val progress = timeLeft / timeoutSeconds.toFloat()
			val animatedProgress by animateFloatAsState(
				targetValue = progress,
				animationSpec = ProgressIndicatorDefaults.ProgressAnimationSpec,
			)
			CircularProgressIndicator(
				progress = animatedProgress,
				color = MaterialTheme.colors.pink300,
			)
			Text(
				text = timeLeft.toString(),
				color = MaterialTheme.colors.primary,
				style = TypefaceNew.LabelS,
			)
		}
	}
	if (timeLeft <= 0) {
		onTimeOutAction()
	}
	LaunchedEffect(key1 = Unit) {
		while (timeLeft > 0) {
			delay(1.seconds)
			timeLeft -= 1
		}
	}
}


@Preview
@Composable
private fun PreviewCountDown() {
	SignerNewTheme() {
		val context = LocalContext.current
		CircularCountDownTimer(timeoutSeconds = 10, text = "Time is going out") {
			Toast.makeText(context, "time is up", Toast.LENGTH_SHORT).show()
		}
	}
}
