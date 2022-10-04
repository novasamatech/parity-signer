package io.parity.signer.ui

import androidx.activity.compose.BackHandler
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.wrapContentHeight
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import io.parity.signer.ui.theme.backgroundTertiary
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
	var wasSheetShown by remember { mutableStateOf(false) }

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
		sheetBackgroundColor = MaterialTheme.colors.backgroundTertiary,
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

	//show once view is create to have initial open animation
	LaunchedEffect(key1 = modalBottomSheetState) {
		modalBottomSheetState.show()
		wasSheetShown = true
	}

	// Take action based on hidden state
	LaunchedEffect(modalBottomSheetState.currentValue) {
		when (modalBottomSheetState.currentValue) {
			ModalBottomSheetValue.Hidden -> {
				if (!wasSheetClosed && wasSheetShown) {
					onClosedAction()
				}
			}
			else -> {}
		}
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
	) {
		content()
	}
}
