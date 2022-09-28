package io.parity.signer.ui

import androidx.activity.compose.BackHandler
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.wrapContentHeight
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
import com.google.accompanist.insets.navigationBarsWithImePadding
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
			ModalBottomSheetValue.Expanded,
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
			.navigationBarsWithImePadding()
			.wrapContentHeight()
			.fillMaxWidth()
			.clip(RoundedCornerShape(topEnd = 16.dp, topStart = 16.dp))
			.background(Color.White)
	) {
			content()
	}
}
