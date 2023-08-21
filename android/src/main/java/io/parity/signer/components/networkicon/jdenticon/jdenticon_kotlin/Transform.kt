package io.parity.signer.components.networkicon.jdenticon.jdenticon_kotlin

internal class Transform(val x_t: Float,
												 val y_t: Float,
												 val size_t: Float,
												 val rotation_t: Float
) {

    fun transformPoint(x: Float, y: Float, w: Float? = null, h: Float? = null): Point {
        val right = this.x_t + this.size_t
        val bottom = this.y_t + this.size_t
        val height = h ?: 0f
        val width = w ?: 0f
        return if (this.rotation_t == 1f) Point(right - y - height, this.y_t + x) else
            if (this.rotation_t == 2f) Point(right - x - width, bottom - y - height) else
                if (this.rotation_t == 3f) Point(this.x_t + y, bottom - x - width) else
                    Point(this.x_t + x, this.y_t + y)
    }

    companion object {
        fun noTransform(): Transform {
            return Transform(0f, 0f, 0f, 0f)
        }
    }
}
