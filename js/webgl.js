var gl_context = null

export function gl_load_context(canvas_name) {
    gl_context = document.getElementById(canvas_name).getContext('webgl');
}

export function gl_enable(capability) {
    gl_context.enable(capability);
}
export function gl_blend_func(sfactor, dfactor) {
    gl_context.blendFunc(sfactor, dfactor);
}
export function gl_draw_arrays(mode, first, count) {
    gl_context.drawArrays(mode, first, count);
}
export function gl_clear_color(r, g, b, a) {
    gl_context.clearColor(r, g, b, a);
}
export function gl_clear(mask) {
    gl_context.clear(mask);
}

export function gl_create_texture() {
    return gl_context.createTexture();
}
export function gl_delete_texture(texture) {
    gl_context.deleteTexture(texture);
}
export function gl_bind_texture(target, texture) {
    gl_context.bindTexture(target, texture);
};
export function gl_active_texture(texture) {
    gl_context.activeTexture(texture);
}
export function gl_tex_parameter_i(target, pname, param) {
    gl_context.texParameteri(target, pname, param);
}

export function gl_tex_image_2d_empty(target, level, internalFormat, width, height, border, format, data_type, data) {
    if (data_type == WebGLRenderingContext.UNSIGNED_BYTE && format == WebGLRenderingContext.RGBA) {
        var pixels = new Uint8Array(width * height * 4);
        gl_context.texImage2D(target, level, internalFormat, width, height, border, format, data_type, pixels);
    } else {
        throw "Only format of RGBA and data type of UNSIGNED_BYTE supported"
    }
}
// Partially implemented
export function gl_tex_image_2d(target, level, internalFormat, width, height, border, format, data_type, pixels) {
    if (data_type == WebGLRenderingContext.UNSIGNED_BYTE && format == WebGLRenderingContext.RGBA) {
        gl_context.texImage2D(target, level, internalFormat, width, height, border, format, data_type, pixels);
    } else {
        throw "Only format of RGBA and data type of UNSIGNED_BYTE supported"
    }
}
// Partially implemented
export function gl_tex_sub_image_2d(target, level, xoffset, yoffset, width, height, format, data_type, pixels) {
    if (data_type == WebGLRenderingContext.UNSIGNED_BYTE && format == WebGLRenderingContext.RGBA) {
        gl_context.texSubImage2D(target, level, xoffset, yoffset, width, height, format, data_type, pixels);
    } else {
        throw "Only format of RGBA and data type of UNSIGNED_BYTE supported"
    }
}

export function gl_create_shader(type) {
    return gl_context.createShader(type);
}
export function gl_delete_shader(shader) {
    gl_context.deleteShader(shader);
}
export function gl_shader_source(shader, source) {
    gl_context.shaderSource(shader, source);
}
export function gl_compile_shader(shader) {
    gl_context.compileShader(shader);
}
export function gl_get_shader_parameter(shader, pname) {
    return gl_context.getShaderParameter(shader, pname);
};
export function gl_shader_info_log_len(shader) {
    const utf8Encoder = new TextEncoder("UTF-8");
    let string_buffer = utf8Encoder.encode(gl_context.getShaderInfoLog(shader))
    return string_buffer.length
}
export function gl_get_shader_info_log(shader, size, str_ptr) {
    const utf8Encoder = new TextEncoder("UTF-8");
    let string_buffer = utf8Encoder.encode(gl_context.getShaderInfoLog(shader))
    for (i = 0; i < size; i++) {
        HEAPU8[str_ptr + i] = string_buffer[i]
    }
}

export function gl_create_program() {
    var ref = gl_context.createProgram();
    return ref;
}
export function gl_delete_program(program) {
    gl_context.deleteProgram(program);
}
export function gl_attach_shader(program, shader) {
    gl_context.attachShader(program, shader);
}
export function gl_link_program(program) {
    gl_context.linkProgram(program);
}
export function gl_use_program(program) {
    gl_context.useProgram(program);
}
export function gl_get_program_parameter(program, pname) {
    return gl_context.getProgramParameter(program, pname);
}
export function gl_program_info_log_len(program) {
    let string_buffer = utf8Encoder.encode(gl_context.getProgramInfoLog(program))
    return string_buffer.length
}
export function gl_get_program_info_log(program) {
    return gl_context.getProgramInfoLog(program);
}

export function gl_create_buffer() {
    return gl_context.createBuffer();
}
export function gl_delete_buffer(buffer) {
    gl_context.deleteBuffer(buffer);
}
export function gl_bind_buffer(target, buffer) {
    gl_context.bindBuffer(target, buffer);
}
export function gl_buffer_data(target, data, usage) {
    gl_context.bufferData(target, data, usage);
}

export function gl_get_uniform_location(program, name) {
    var location = gl_context.getUniformLocation(program, name);
    if (location == null) {
        throw "Could not find uniform '" + name + "'";
    }
    return location;
};
export function gl_uniform2f(location, v0, v1) {
    gl_context.uniform2f(location, v0, v1);
}
export function gl_uniform1i(location, v0) {
    gl_context.uniform1i(location, v0);
}

export function gl_get_attrib_location(program, name) {
    var location = gl_context.getAttribLocation(program, name);
    return location;
};
export function gl_enable_vertex_attrib_array(index) {
    gl_context.enableVertexAttribArray(index);
}
export function gl_vertex_attrib_pointer(index, size, type, normalized, stride, offset) {
    gl_context.vertexAttribPointer(index, size, type, normalized, stride, offset);
}