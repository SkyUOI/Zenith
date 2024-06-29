shader_type canvas_item;

void vertex() {
	// Called for every vertex the material is visible on.
}

void fragment() {
	// Called for every pixel the material is visible on.
	vec2 screen_coords = SCREEN_UV;
	// 垂直翻转Y坐标
	screen_coords.y = (1.0 - screen_coords.y); 
	// 紫色
	vec4 purple_color = vec4(0.5, 0.0, 0.5, 1.0); 
	// 半透明紫色
	vec4 transparent_purple = vec4(0.5, 0.0, 0.5, 0.5); 
	vec4 gradient_color = purple_color + (transparent_purple - purple_color) * screen_coords.y * 3.6;
	// 设置最终颜色
	COLOR = gradient_color;
}

//void light() {
	// Called for every pixel for every light affecting the CanvasItem.
	// Uncomment to replace the default light processing function with this one.
//}
