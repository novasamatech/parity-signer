package io.parity.signer.screens.initial.explanation

import androidx.appcompat.app.AppCompatDelegate
import androidx.compose.foundation.ExperimentalFoundationApi
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.captionBarPadding
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.navigationBarsPadding
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.foundation.pager.HorizontalPager
import androidx.compose.foundation.pager.rememberPagerState
import androidx.compose.runtime.Composable
import androidx.compose.runtime.DisposableEffect
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.ui.Modifier
import io.parity.signer.domain.Callback
import kotlinx.coroutines.launch


@OptIn(ExperimentalFoundationApi::class)
@Composable
fun OnboardingExplanationScreenFull(navigateNext: Callback) {
	val scope = rememberCoroutineScope()
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
		//actual screens
		HorizontalPager(state = pagerState) {
			when (it) {
				0 -> OnboardingScreen1(navigateNext)
				1 -> OnboardingScreen2(navigateNext)
				2 -> OnboardingScreen3(navigateNext)
				3 -> OnboardingScreen4(navigateNext)
			}
		}
		//things to handle clicks for scrolling
		Row(Modifier.fillMaxSize(1f)) {
			Box(
				Modifier
					.weight(0.3f)
					.fillMaxHeight()
					.clickable {
						//back action area
						scope.launch {
							if (pagerState.canScrollBackward) {
								pagerState.animateScrollToPage(pagerState.currentPage - 1)
							}
						}
					}
			)
			Box(Modifier.weight(0.6f))
			Box(
				Modifier
					.weight(0.3f)
					.fillMaxHeight()
					.clickable {
						//back action area
						scope.launch {
							if (pagerState.canScrollForward) {
								pagerState.animateScrollToPage(pagerState.currentPage + 1)
							}
						}
					})
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
