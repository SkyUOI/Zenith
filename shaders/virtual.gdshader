shader_type canvas_item;

uniform vec4 color;
uniform float alpha_end;
uniform float alpha_start;
uniform float end_y_position;

void vertex() {
	// Called for every vertex the material is visible on.
}

void fragment() {
	// Called for every pixel the material is visible on.
	vec2 screen_coords = SCREEN_UV;
	if(screen_coords.y < end_y_position) {
		COLOR = vec4(0,0,0,0);
	} else {
		// 垂直翻转Y坐标
		screen_coords.y = (1.0 - screen_coords.y);
		COLOR = color;
		COLOR.a = alpha_start + (alpha_end - alpha_start) * screen_coords.y / (end_y_position / 1.5);
	}
}

//void light() {
	// Called for every pixel for every light affecting the CanvasItem.
	// Uncomment to replace the default light processing function with this one.
//}
