#![feature(collections)]
#![no_std]


extern crate collections;

extern crate vector;
extern crate stack;
extern crate hash_map;
extern crate insert;
extern crate map;
extern crate iterable;

extern crate camera_components;

extern crate gl;
extern crate gl_geometry;
extern crate gl_context;

extern crate geometry;
extern crate material;

extern crate uuid;

extern crate scene_graph;
extern crate scene_renderer;

extern crate shared;


mod gl_renderer_plugin;


pub use gl_renderer_plugin::GLRendererPlugin;
