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
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.layout.onGloballyPositioned
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.domain.conditional
import io.parity.signer.domain.submitErrorState
import io.parity.signer.ui.theme.*
import kotlinx.coroutines.delay
import kotlin.time.Duration.Companion.seconds

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
			style = SignerTypeface.BodyL,
		)
		Box(
			modifier = Modifier.padding(start = 8.dp),
			contentAlignment = Alignment.Center
		) {
			val progress = timeLeft / timeoutSeconds.toFloat()
			val animatedProgress by animateFloatAsState(
				targetValue = progress,
				animationSpec = ProgressIndicatorDefaults.ProgressAnimationSpec,
				label = "circular countdown animation",
			)
			CircularProgressIndicator(
				progress = animatedProgress,
				color = MaterialTheme.colors.pink300,
			)
			Text(
				text = timeLeft.toString(),
				color = MaterialTheme.colors.primary,
				style = SignerTypeface.LabelS,
			)
		}
	}

	val currentTimeoutAction by rememberUpdatedState(onTimeOutAction)
	LaunchedEffect(key1 = Unit) {
		try {
			while (timeLeft > 0) {
				delay(1.seconds)
				timeLeft -= 1
				if (timeLeft == 0) {
					currentTimeoutAction()
				}
			}
		} finally {
			if (timeLeft > 0) {
				currentTimeoutAction()
			}
		}
	}
}


@Composable
fun SnackBarCircularCountDownTimer(
	timeoutSeconds: Int,
	text: String,
	componentHeighCallback: MutableState<Dp>? = null,
	modifier: Modifier = Modifier,
	onTimeOutAction: () -> Unit,
) {
	if (timeoutSeconds <= 0) {
		onTimeOutAction()
		submitErrorState("timer started with wrong value $timeoutSeconds")
	}

	val innerShape =
		RoundedCornerShape(dimensionResource(id = R.dimen.innerFramesCornerRadius))
	val localDensity = LocalDensity.current

	var timeLeft by remember { mutableStateOf<Int>(timeoutSeconds) }
	Row(
		modifier = modifier
			.conditional(componentHeighCallback != null) {
				onGloballyPositioned { coordinates ->
					componentHeighCallback?.value =
						with(localDensity) { coordinates.size.height.toDp() }
				}
			}
			.fillMaxWidth(1f)
			.padding(start = 8.dp, top = 8.dp, end = 8.dp, bottom = 16.dp)
			.background(MaterialTheme.colors.snackBarBackground, innerShape)
			.padding(12.dp),
		verticalAlignment = Alignment.CenterVertically
	) {
		Text(
			text = text,
			color = Color.White,
			style = SignerTypeface.BodyL,
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
				label = "snack bar countdown animation",
			)
			CircularProgressIndicator(
				progress = animatedProgress,
				color = MaterialTheme.colors.pink300,
			)
			Text(
				text = timeLeft.toString(),
				color = Color.White,
				style = SignerTypeface.LabelS,
			)
		}
	}


	val currentTimeoutAction by rememberUpdatedState(onTimeOutAction)
	LaunchedEffect(key1 = Unit) {
		while (timeLeft > 0) {
			delay(1.seconds)
			timeLeft -= 1
			if (timeLeft == 0) {
				currentTimeoutAction()
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
