package io.parity.signer.screens.initial.explanation

import android.content.res.Configuration
import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.graphics.painter.Painter
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import io.parity.signer.R
import io.parity.signer.components.base.PageIndicatorLine
import io.parity.signer.domain.Callback
import io.parity.signer.ui.theme.SignerNewTheme
import io.parity.signer.ui.theme.SignerTypeface
import io.parity.signer.ui.theme.backgroundSecondary
import io.parity.signer.ui.theme.fill12
import io.parity.signer.ui.theme.pink500
import io.parity.signer.ui.theme.textSecondary


@Composable
internal fun OnboardingScreen1(onSkip: Callback) {
	OnboardingScreenGeneric(
		page = 1,
		showSkip = true,
		title = stringResource(R.string.onboarding_header_1),
		image = painterResource(id = R.drawable.onboarding_1),
		onSkip = onSkip,
	)
}

@Composable
internal fun OnboardingScreenGeneric(
	page: Int,
	showSkip: Boolean,
	title: String,
	image: Painter,
	onSkip: Callback
) {
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
			selectedIndex = page,
			modifier = Modifier.padding(horizontal = 16.dp, vertical = 16.dp),
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
			if (showSkip) {
				Text(
					stringResource(R.string.onboarding_skip),
					color = MaterialTheme.colors.primary,
					style = SignerTypeface.LabelS,
					modifier = Modifier
						.padding(horizontal = 6.dp)
						.clickable(onClick = onSkip)
				)
			}
		}
		Spacer(modifier = Modifier.weight(0.05f))
		Text(
			text = title,
			color = MaterialTheme.colors.primary,
			style = SignerTypeface.TitleM,
			modifier = Modifier.padding(horizontal = 24.dp, vertical = 8.dp)
		)
		Spacer(
			modifier = Modifier
				.padding(top = 30.dp)
				.weight(0.2f)
		)
		Image(
			painter = image,
			contentDescription = null,
			modifier = Modifier
				.padding(horizontal = 24.dp)
				.fillMaxWidth(1f)
				.weight(0.7f)
		)
		Spacer(
			modifier = Modifier
				.padding(top = 16.dp)
				.weight(0.2f)
		)
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

