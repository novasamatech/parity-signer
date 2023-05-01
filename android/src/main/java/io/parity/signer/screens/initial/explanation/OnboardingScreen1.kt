package io.parity.signer.screens.initial.explanation

import android.content.res.Configuration
import androidx.appcompat.app.AppCompatDelegate
import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.DisposableEffect
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.PageIndicatorLine
import io.parity.signer.domain.Callback
import io.parity.signer.ui.theme.*


@Composable
internal fun OnboardingScreen1(onSkip: Callback) {
	ForceDarkTheme()
	Column(
		Modifier
			.fillMaxSize(1f)
			.background(
				brush = Brush.linearGradient(
					start = Offset(Float.POSITIVE_INFINITY, 0.0f),
					end = Offset(0.0f, Float.POSITIVE_INFINITY),
					colors = listOf(
						MaterialTheme.colors.pink500,
						MaterialTheme.colors.backgroundSecondary,
					),
				)
			)
	) {
		PageIndicatorLine(
			totalDots = 4,
			selectedIndex = 1,
			modifier = Modifier.padding(horizontal = 16.dp, vertical = 8.dp),
		)
		Row(
			modifier = Modifier.padding(vertical = 8.dp, horizontal = 16.dp),
			verticalAlignment = Alignment.CenterVertically,
		) {
			Box(
				modifier = Modifier
					.background(
						MaterialTheme.colors.fill12, RoundedCornerShape(40.dp)
					)
					.padding(vertical = 6.dp, horizontal = 16.dp),
			) {
				Text(
					stringResource(R.string.onboarding_welcome),
					color = MaterialTheme.colors.textSecondary,
					style = SignerTypeface.LabelS,
				)
			}
			Spacer(modifier = Modifier.weight(1f))
			Text(
				stringResource(R.string.onboarding_skip),
				color = MaterialTheme.colors.primary,
				style = SignerTypeface.LabelS,
				modifier = Modifier
					.padding(horizontal = 6.dp)
					.clickable(onClick = onSkip)
			)
		}
		Text(
			text = stringResource(R.string.onboarding_header_1),
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.TitleS,
			modifier = Modifier.padding(horizontal = 24.dp, vertical = 8.dp)
		)
		Spacer(
			modifier = Modifier
				.padding(top = 30.dp)
				.weight(0.2f)
		)
		Image(
			painter = painterResource(id = R.drawable.onboarding_2),
			contentDescription = null,
			modifier = Modifier
				.padding(horizontal = 24.dp)
				.fillMaxWidth(1f)
				.weight(0.7f)
		)
		Spacer(modifier = Modifier.padding(top = 16.dp))
	}
}

@Composable
fun ForceDarkTheme() {
	DisposableEffect(key1 = Unit) {
		val original = AppCompatDelegate.getDefaultNightMode()
		AppCompatDelegate.setDefaultNightMode(AppCompatDelegate.MODE_NIGHT_YES)
		onDispose {
			AppCompatDelegate.setDefaultNightMode(original)
		}
	}
}


@Preview(
	name = "dark", group = "themes", uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun PreviewOnboarding1Small() {
	SignerNewTheme {
		Box(modifier = Modifier.size(320.dp, 568.dp)) {
			OnboardingScreen1({})
		}
	}
}

@Preview(
	name = "dark", group = "themes", uiMode = Configuration.UI_MODE_NIGHT_YES,
	showBackground = true, backgroundColor = 0xFF000000,
)
@Composable
private fun PreviewOnboarding1Big() {
	SignerNewTheme {
		Box(modifier = Modifier.fillMaxSize(1f)) {
			OnboardingScreen1({})
		}
	}
}

