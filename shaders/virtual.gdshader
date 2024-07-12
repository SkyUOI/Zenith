shader_type canvas_item;

uniform vec4 color_end;
uniform vec4 color_start;
uniform float alpha_bias = 0;

void vertex() {
	// Called for every vertex the material is visible on.
}

void fragment() {
	// Called for every pixel the material is visible on.
	vec2 screen_coords = SCREEN_UV;
	// 垂直翻转Y坐标
	screen_coords.y = (1.0 - screen_coords.y);
	vec4 gradient_color = color_start + (color_end - color_start) * screen_coords.y * 3.6;
	// 设置最终颜色
	COLOR = gradient_color;
	COLOR.a -= alpha_bias;
}

//void light() {
	// Called for every pixel for every light affecting the CanvasItem.
	// Uncomment to replace the default light processing function with this one.
//}
