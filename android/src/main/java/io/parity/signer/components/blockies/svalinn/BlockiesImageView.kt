package io.parity.signer.components.blockies.svalinn

import android.content.Context
import android.graphics.Canvas
import android.util.AttributeSet
import androidx.appcompat.widget.AppCompatImageView

/**
 * Based on svalinn-kotlin project which is MIT licensed.
 * todo dmitry remove it?
 */
open class BlockiesImageView(context: Context, attributeSet: AttributeSet?) : AppCompatImageView(context, attributeSet) {

    private var blockies: Blockies? = null
    private var painter: BlockiesPainterOld = BlockiesPainterOld()

    fun setAddress(seed: String) {
        blockies = seed.let { Blockies.fromSeed(seed) }
        invalidate()
    }

    override fun onDraw(canvas: Canvas) {
        super.onDraw(canvas)
        blockies?.let { drawBlockies(canvas, it) }
    }

    override fun onMeasure(widthMeasureSpec: Int, heightMeasureSpec: Int) {
        super.onMeasure(widthMeasureSpec, heightMeasureSpec)
        painter.setDimensions(measuredWidth.toFloat(), measuredHeight.toFloat())
    }

    private fun drawBlockies(canvas: Canvas, blockies: Blockies) {
        painter.draw(canvas, blockies)
    }
}
