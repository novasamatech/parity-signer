package io.parity.signer.components.sharedcomponents

import android.widget.Toast
import androidx.compose.animation.core.animateFloatAsState
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.CircularProgressIndicator
import androidx.compose.material.MaterialTheme
import androidx.compose.material.ProgressIndicatorDefaults
import androidx.compose.material.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.models.submitErrorState
import io.parity.signer.ui.theme.*
import kotlinx.coroutines.delay
import kotlin.time.ExperimentalTime
import kotlin.time.seconds

@OptIn(ExperimentalTime::class)
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

	LaunchedEffect(key1 = Unit) {
		while (timeLeft > 0) {
			delay(1.seconds)
			timeLeft -= 1
			if (timeLeft == 0) {
				onTimeOutAction()
			}
		}
	}
}


@OptIn(ExperimentalTime::class)
@Composable
fun SnackBarCircularCountDownTimer(
	timeoutSeconds: Int,
	text: String,
	modifier: Modifier = Modifier,
	onTimeOutAction: () -> Unit
) {
	if (timeoutSeconds <= 0) {
		onTimeOutAction()
		submitErrorState("timer started with wrong value $timeoutSeconds")
	}

	val innerRoun = dimensionResource(id = R.dimen.innerFramesCornerRadius)
	val innerShape =
		RoundedCornerShape(innerRoun, innerRoun, innerRoun, innerRoun)

	var timeLeft by remember { mutableStateOf<Int>(timeoutSeconds) }
	Row(
		modifier = modifier
			.fillMaxWidth(1f)
			.padding(horizontal = 8.dp, vertical = 16.dp)
			.background(MaterialTheme.colors.snackBarBackground, innerShape)
			.padding(12.dp),
		verticalAlignment = Alignment.CenterVertically
	) {
		Text(
			text = text,
			color = MaterialTheme.colors.textSecondary,
			style = TypefaceNew.BodyL,
			modifier = Modifier.weight(1f)
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

	LaunchedEffect(key1 = Unit) {
		while (timeLeft > 0) {
			delay(1.seconds)
			timeLeft -= 1
			if (timeLeft == 0) {
				onTimeOutAction()
			}
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

@Preview
@Composable
private fun PreviewSnackCountDown() {
	SignerNewTheme() {
		val context = LocalContext.current
		SnackBarCircularCountDownTimer(
			timeoutSeconds = 10,
			text = "Time is going out"
		) {
			Toast.makeText(context, "time is up", Toast.LENGTH_SHORT).show()
		}
	}
}
