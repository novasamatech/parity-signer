package io.parity.signer.ui

import androidx.activity.compose.BackHandler
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.wrapContentHeight
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.*
import androidx.compose.runtime.Composable
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.launch


//todo dmitry handle closing bottom sheet as need to press back

@OptIn(ExperimentalMaterialApi::class)
@Composable
fun BottomSheetWrapper(
	bottomSheetContent: @Composable (state: BottomSheetPositionHandle) -> Unit,
) {
	val coroutineScope = rememberCoroutineScope()
	val modalBottomSheetState =
		rememberModalBottomSheetState(
			ModalBottomSheetValue.Hidden,
			confirmStateChange = {
				it != ModalBottomSheetValue.HalfExpanded
			}
		)

	val handle = object : BottomSheetPositionHandle {
		override fun hide() {
			coroutineScope.launch {
				if (!modalBottomSheetState.isVisible) {
					modalBottomSheetState.hide()
				}
			}
		}

		override fun show() {
			coroutineScope.launch {
				if (modalBottomSheetState.isVisible) {
					modalBottomSheetState.hide()
				}
				modalBottomSheetState.show()
			}
		}
	}

	ModalBottomSheetLayout(
		sheetBackgroundColor = Color.Transparent,
		sheetState = modalBottomSheetState,
		sheetContent = {
			BottomSheetContentWrapper(
				coroutineScope,
				modalBottomSheetState
			) {
				bottomSheetContent(handle)
			}
		},
		content = {},
	)

	BackHandler {
		coroutineScope.launch { modalBottomSheetState.hide() }
	}
}

interface BottomSheetPositionHandle {
	fun hide()
	fun show()
}

@OptIn(ExperimentalMaterialApi::class)
@Composable
private fun BottomSheetContentWrapper(
	coroutineScope: CoroutineScope,
	modalBottomSheetState: ModalBottomSheetState,
	content: @Composable () -> Unit,
) {
	Box(
		modifier = Modifier
			.wrapContentHeight()
			.fillMaxWidth()
			.clip(RoundedCornerShape(topEnd = 16.dp, topStart = 16.dp))
			.background(Color.White)
	) {
		Box(Modifier.padding(top = 25.dp)) {
			content()
		}
	}
}
