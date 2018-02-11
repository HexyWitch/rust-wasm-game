function reference_allocator() {
    var refs = [];
    var free = [];

    function create(item) {
        var key = free.pop();
        if (key == undefined) {
            key = refs.push(item) - 1;
        } else {
            refs[key] = item;
        }
        return key
    }

    function remove(key) {
        refs[key] = null;
        free.push(key);
    }

    function get(key) {
        return refs[key];
    }

    function take(key) {
        var handle = get(key);
        remove(key);
        return handle;
    }

    return {
        create: create,
        remove: remove,
        get: get,
        take: take,
    }
}

const gl = {
    context: null,

    refs: reference_allocator()
}

function gl_enable(capability) {
    gl.context.enable(capability);
}
function gl_blend_func(sfactor, dfactor) {
    gl.context.blendFunc(sfactor, dfactor);
}
function gl_draw_arrays(mode, first, count) {
    gl.context.drawArrays(mode, first, count);
}
function gl_clear_color(r, g, b, a) {
    gl.context.clearColor(r, g, b, a);
}
function gl_clear(mask) {
    gl.context.clear(mask);
}

function gl_create_texture() {
    return gl.refs.create(gl.context.createTexture());
}
function gl_delete_texture(texture_ref) {
    var texture = gl.refs.take(texture_ref);
    gl.context.deleteTexture(shader);
}
function gl_bind_texture(target, texture_ref) {
    var texture = gl.refs.get(texture_ref);
    gl.context.bindTexture(target, texture);
};
function gl_active_texture(texture) {
    gl.context.activeTexture(texture);
}
function gl_tex_parameter_i(target, pname, param) {
    gl.context.texParameteri(target, pname, param);
}
// Partially implemented
function gl_tex_image_2d(target, level, internalFormat, width, height, border, format, data_type, pixels_ptr) {
    if (data_type == WebGLRenderingContext.UNSIGNED_BYTE && format == WebGLRenderingContext.RGBA) {

        const buffer = Module.instance.exports.memory.buffer;
        var pixels;
        if (pixels_ptr == 0) {
            pixels = new Uint8Array(width * height * 4)
        } else {
            pixels = new Uint8Array(buffer, pixels_ptr, width * height * 4)
        }
        gl.context.texImage2D(target, level, internalFormat, width, height, border, format, data_type, pixels);
    } else {
        throw "Only format of RGBA and data type of UNSIGNED_BYTE supported"
    }
}
// Partially implemented
function gl_tex_sub_image_2d(target, level, xoffset, yoffset, width, height, format, data_type, pixels_ptr) {
    if (data_type == WebGLRenderingContext.UNSIGNED_BYTE && format == WebGLRenderingContext.RGBA) {
        const buffer = Module.instance.exports.memory.buffer;
        var pixels;
        if (pixels_ptr == 0) {
            pixels = new Uint8Array(width * height * 4)
        } else {
            pixels = new Uint8Array(buffer, pixels_ptr, width * height * 4)
        }
        gl.context.texSubImage2D(target, level, xoffset, yoffset, width, height, format, data_type, pixels);
    } else {
        throw "Only format of RGBA and data type of UNSIGNED_BYTE supported"
    }
}

function gl_create_shader(type) {
    return gl.refs.create(gl.context.createShader(type));
}
function gl_delete_shader(shader_ref) {
    var shader = gl.refs.take(shader_ref);
    gl.context.deleteShader(shader);
}
function gl_shader_source(shader_ref, source_ptr) {
    var shader = gl.refs.get(shader_ref);
    var source = copyCStr(source_ptr);
    gl.context.shaderSource(shader, source);
}
function gl_compile_shader(shader_ref) {
    var shader = gl.refs.get(shader_ref);
    gl.context.compileShader(shader);
}
function gl_get_shader_parameter(shader_ref, pname) {
    var shader = gl.refs.get(shader_ref);
    return gl.context.getShaderParameter(shader, pname);
};
function gl_shader_info_log_len(shader_ref) {
    var shader = gl.refs.get(shader_ref);
    const utf8Encoder = new TextEncoder("UTF-8");
    let string_buffer = utf8Encoder.encode(gl.context.getShaderInfoLog(shader))
    return string_buffer.length
}
function gl_get_shader_info_log(shader_ref, size, str_ptr) {
    var shader = gl.refs.get(shader_ref);
    const utf8Encoder = new TextEncoder("UTF-8");
    let string_buffer = utf8Encoder.encode(gl.context.getShaderInfoLog(shader))
    for (i = 0; i < size; i++) {
        HEAPU8[str_ptr + i] = string_buffer[i]
    }
}

function gl_create_program() {
    var ref = gl.refs.create(gl.context.createProgram());
    return ref;
}
function gl_delete_program(program_ref) {
    var program = gl.refs.take(program_ref);
    gl.context.deleteProgram(program);
}
function gl_attach_shader(program_ref, shader_ref) {
    var program = gl.refs.get(program_ref);
    var shader = gl.refs.get(shader_ref);
    gl.context.attachShader(program, shader);
}
function gl_link_program(program_ref) {
    var program = gl.refs.get(program_ref);
    gl.context.linkProgram(program);
}
function gl_use_program(program_ref) {
    var program = gl.refs.get(program_ref);
    gl.context.useProgram(program);
}
function gl_get_program_parameter(program_ref, pname) {
    var program = gl.refs.get(program_ref);
    return gl.context.getProgramParameter(program, pname);
}
function gl_program_info_log_len(program_ref) {
    var program = gl.refs.get(program_ref);
    let string_buffer = utf8Encoder.encode(gl.context.getProgramInfoLog(program))
    return string_buffer.length
}
function gl_get_program_info_log(program_ref, size, str_ptr) {
    var program = gl.refs.get(program_ref);
    let string_buffer = utf8Encoder.encode(gl.context.getProgramInfoLog(program))
    if (size != string_buffer.length) {
        throw "mismatched info log length"
    }
    for (i = 0; i < size; i++) {
        HEAPU8[str_ptr + i] = string_buffer[i]
    }
}

function gl_create_buffer() {
    return gl.refs.create(gl.context.createBuffer());
}
function gl_delete_buffer(buffer_ref) {
    var buffer = gl.refs.take(buffer_ref);
    gl.context.deleteBuffer(buffer);
}
function gl_bind_buffer(target, buffer_ref) {
    var buffer = gl.refs.get(buffer_ref);
    gl.context.bindBuffer(target, buffer);
}
function gl_buffer_data(target, size, data_ptr, usage) {
    const buffer = Module.instance.exports.memory.buffer;
    var data = new Uint8Array(buffer, data_ptr, size);
    gl.context.bufferData(target, data, usage);
}

function gl_get_uniform_location(program_ref, name_ptr) {
    var program = gl.refs.get(program_ref);
    var name = copyCStr(name_ptr);
    var location = gl.context.getUniformLocation(program, name);
    if (location == null) {
        throw "Could not find uniform '" + name + "'";
    }
    return gl.refs.create(location);
};
function gl_delete_uniform_location(location_ref) {
    gl.refs.remove(location_ref);
}
function gl_uniform2f(location_ref, v0, v1) {
    var location = gl.refs.get(location_ref);
    gl.context.uniform2f(location, v0, v1);
}
function gl_uniform1i(location_ref, v0) {
    var location = gl.refs.get(location_ref);
    gl.context.uniform1i(location, v0);
}

function gl_get_attrib_location(program_ref, name_ptr) {
    var program = gl.refs.get(program_ref);
    var name = copyCStr(name_ptr);
    var location = gl.context.getAttribLocation(program, name);
    return location;
};
function gl_enable_vertex_attrib_array(index) {
    gl.context.enableVertexAttribArray(index);
}
function gl_vertex_attrib_pointer(index, size, type, normalized, stride, offset) {
    gl.context.vertexAttribPointer(index, size, type, normalized, stride, offset);
}