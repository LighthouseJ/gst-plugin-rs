//  Copyright (C) 2016 Sebastian Dröge <sebastian@centricular.com>
//
//  This library is free software; you can redistribute it and/or
//  modify it under the terms of the GNU Library General Public
//  License as published by the Free Software Foundation; either
//  version 2 of the License, or (at your option) any later version.
//
//  This library is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
//  Library General Public License for more details.
//
//  You should have received a copy of the GNU Library General Public
//  License along with this library; if not, write to the
//  Free Software Foundation, Inc., 51 Franklin St, Fifth Floor,
//  Boston, MA 02110-1301, USA.

#![crate_type="cdylib"]

extern crate url;
#[macro_use]
extern crate gst_plugin;
#[macro_use]
extern crate slog;
#[macro_use]
extern crate nom;
extern crate flavors;

use gst_plugin::plugin::*;
use gst_plugin::demuxer::*;

mod flvdemux;

use flvdemux::FlvDemux;

fn plugin_init(plugin: &Plugin) -> bool {
    demuxer_register(plugin,
                     &DemuxerInfo {
                         name: "rsflvdemux",
                         long_name: "FLV Demuxer",
                         description: "Demuxes FLV Streams",
                         classification: "Codec/Demuxer",
                         author: "Sebastian Dröge <sebastian@centricular.com>",
                         rank: 256 + 100,
                         create_instance: FlvDemux::new_boxed,
                         input_formats: "video/x-flv",
                         output_formats: "ANY",
                     });

    true
}

plugin_define!(b"rsflv\0",
               b"Rust FLV Plugin\0",
               plugin_init,
               b"1.0\0",
               b"LGPL\0",
               b"rsflv\0",
               b"rsflv\0",
               b"https://github.com/sdroege/rsplugin\0",
               b"2016-12-08\0");