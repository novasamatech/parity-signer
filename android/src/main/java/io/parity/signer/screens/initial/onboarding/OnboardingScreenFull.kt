package io.parity.signer.screens.initial.onboarding

import androidx.compose.foundation.ExperimentalFoundationApi
import androidx.compose.foundation.pager.HorizontalPager
import androidx.compose.runtime.Composable
import io.parity.signer.domain.Callback


@OptIn(ExperimentalFoundationApi::class)
@Composable
fun OnboardingScreenFull(navigateNext: Callback) {
	HorizontalPager(pageCount = 4) {
		when (it) {
			1 -> OnboardingScreen1(navigateNext)
			2 -> OnboardingScreen2(navigateNext)
			3 -> OnboardingScreen3(navigateNext)
			4 -> OnboardingScreen4(navigateNext)
		}
	}
}
