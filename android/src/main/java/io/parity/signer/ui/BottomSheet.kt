package io.parity.signer.ui

import androidx.activity.compose.BackHandler
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.ExperimentalMaterialApi
import androidx.compose.material.ModalBottomSheetLayout
import androidx.compose.material.ModalBottomSheetValue
import androidx.compose.material.rememberModalBottomSheetState
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.launch


@OptIn(ExperimentalMaterialApi::class)
@Composable
fun BottomSheetWrapper(
	onClosedAction: () -> Unit = {},
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

	var wasSheetClosed by remember { mutableStateOf(false) }

	val handle = remember {
		object : BottomSheetPositionHandle {
			override fun hide() {
				wasSheetClosed = true
				coroutineScope.launch {
					if (!modalBottomSheetState.isVisible) {
						modalBottomSheetState.hide()
					}
				}
			}

			override fun show() {
				coroutineScope.launch {
					modalBottomSheetState.show()
				}
			}
		}
	}

	ModalBottomSheetLayout(
		sheetBackgroundColor = Color.Transparent,
		sheetState = modalBottomSheetState,
		sheetContent = {
			BottomSheetContentWrapper {
				bottomSheetContent(handle)
			}
		},
		content = {},
	)

	BackHandler {
		coroutineScope.launch { modalBottomSheetState.hide() }
	}

	// Take action based on hidden state
	LaunchedEffect(modalBottomSheetState.currentValue) {
		when (modalBottomSheetState.currentValue) {
			ModalBottomSheetValue.Hidden -> if (!wasSheetClosed) onClosedAction()
			else -> {}
		}
	}

	//show once view is create to have initial open animation
	LaunchedEffect(key1 = modalBottomSheetState) {
			modalBottomSheetState.show()
	}
}

interface BottomSheetPositionHandle {
	fun hide()
	fun show()
}

@Composable
private fun BottomSheetContentWrapper(
	content: @Composable () -> Unit,
) {
	Box(
		modifier = Modifier
			.wrapContentHeight()
			.fillMaxWidth()
			.clip(RoundedCornerShape(topEnd = 16.dp, topStart = 16.dp))
			.background(Color.White)
	) {
			content()
	}
}
