package io.parity.signer.screens.initial.explanation

import androidx.appcompat.app.AppCompatDelegate
import androidx.compose.foundation.ExperimentalFoundationApi
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.captionBarPadding
import androidx.compose.foundation.layout.navigationBarsPadding
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.foundation.pager.HorizontalPager
import androidx.compose.foundation.pager.rememberPagerState
import androidx.compose.runtime.Composable
import androidx.compose.runtime.DisposableEffect
import androidx.compose.ui.Modifier
import io.parity.signer.domain.Callback


@OptIn(ExperimentalFoundationApi::class)
@Composable
fun OnboardingExplanationScreenFull(navigateNext: Callback) {
	Box(
		modifier = Modifier
			.navigationBarsPadding()
			.captionBarPadding()
			.statusBarsPadding()
	) {
		ForceDarkTheme()
		val pagerState = rememberPagerState(pageCount = {
			4
		})
		HorizontalPager(state = pagerState) {
			when (it) {
				0 -> OnboardingScreen1(navigateNext)
				1 -> OnboardingScreen2(navigateNext)
				2 -> OnboardingScreen3(navigateNext)
				3 -> OnboardingScreen4(navigateNext)
			}
		}
	}
}

@Composable
private fun ForceDarkTheme() {
	DisposableEffect(key1 = Unit) {
		val original = AppCompatDelegate.getDefaultNightMode()
		AppCompatDelegate.setDefaultNightMode(AppCompatDelegate.MODE_NIGHT_YES)
		onDispose {
			AppCompatDelegate.setDefaultNightMode(original)
		}
	}
}
