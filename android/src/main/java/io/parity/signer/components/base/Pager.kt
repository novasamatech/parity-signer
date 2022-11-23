package io.parity.signer.components.base

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyRow
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.material.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import io.parity.signer.ui.theme.fill30
import io.parity.signer.ui.theme.pink300


@Composable
fun DotsIndicator(
	totalDots : Int,
	selectedIndex : Int,
	selectedColor: Color = MaterialTheme.colors.pink300,
	unSelectedColor: Color = MaterialTheme.colors.fill30,
){

	LazyRow(
		modifier = Modifier
			.wrapContentWidth()
			.wrapContentHeight()

	) {

		items(totalDots) { index ->
			if (index == selectedIndex) {
				Box(
					modifier = Modifier
						.size(5.dp)
						.clip(CircleShape)
						.background(selectedColor)
				)
			} else {
				Box(
					modifier = Modifier
						.size(5.dp)
						.clip(CircleShape)
						.background(unSelectedColor)
				)
			}

			if (index != totalDots - 1) {
				Spacer(modifier = Modifier.padding(horizontal = 2.dp))
			}
		}
	}
}
