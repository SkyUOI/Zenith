shader_type canvas_item;

void fragment() {
	vec4 original_color = texture(TEXTURE, UV);
	vec4 color = original_color + original_color + original_color;
	COLOR = mix(color, original_color, 1.0 - original_color.a);
}

//void light() {
	// Called for every pixel for every light affecting the CanvasItem.
	// Uncomment to replace the default light processing function with this one.
//}
