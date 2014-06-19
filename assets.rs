use cairo;
use cairo::{Surface, svg};

pub struct Assets<'a> {
    pub add:               (Surface<'a>, f64, f64),
    pub group:             (Surface<'a>, f64, f64),
    pub group_large:       (Surface<'a>, f64, f64),
    pub group_large_black: (Surface<'a>, f64, f64),
    pub settings:          (Surface<'a>, f64, f64),
    pub transfer:          (Surface<'a>, f64, f64),
    pub arrow_medgrey:     (Surface<'a>, f64, f64),
    pub arrow_white:       (Surface<'a>, f64, f64),
    pub attach:            (Surface<'a>, f64, f64),
    pub call:              (Surface<'a>, f64, f64),
    pub check:             (Surface<'a>, f64, f64),
    pub emoticon:          (Surface<'a>, f64, f64),
    pub no:                (Surface<'a>, f64, f64),
    pub pause:             (Surface<'a>, f64, f64),
    pub sendmessage:       (Surface<'a>, f64, f64),
    pub video:             (Surface<'a>, f64, f64),
    pub online_new:        (Surface<'a>, f64, f64),
    pub offline_new:       (Surface<'a>, f64, f64),
    pub away_new:          (Surface<'a>, f64, f64),
    pub online:            (Surface<'a>, f64, f64),
    pub offline:           (Surface<'a>, f64, f64),
    pub away:              (Surface<'a>, f64, f64),
}

pub struct Cached<'a> {
    pub add:               Surface<'a>,
    pub group:             Surface<'a>,
    pub group_large:       Surface<'a>,
    pub group_large_black: Surface<'a>,
    pub settings:          Surface<'a>,
    pub transfer:          Surface<'a>,
    pub arrow_medgrey:     Surface<'a>,
    pub arrow_white:       Surface<'a>,
    pub attach:            Surface<'a>,
    pub call:              Surface<'a>,
    pub check:             Surface<'a>,
    pub emoticon:          Surface<'a>,
    pub no:                Surface<'a>,
    pub pause:             Surface<'a>,
    pub sendmessage:       Surface<'a>,
    pub video:             Surface<'a>,
    pub online_new:        Surface<'a>,
    pub offline_new:       Surface<'a>,
    pub away_new:          Surface<'a>,
    pub online:            Surface<'a>,
    pub offline:           Surface<'a>,
    pub away:              Surface<'a>,
}

impl<'a> Assets<'a> {
    pub fn new() -> Assets<'a> {
        static svg_add:               &'static [u8] = include_bin!("svgs/cli/add.svg");
        static svg_group:             &'static [u8] = include_bin!("svgs/cli/group.svg");
        static svg_group_large:       &'static [u8] = include_bin!("svgs/cli/group_large.svg");
        static svg_group_large_black: &'static [u8] = include_bin!("svgs/cli/group_large_black.svg");
        static svg_settings:          &'static [u8] = include_bin!("svgs/cli/settings.svg");
        static svg_transfer:          &'static [u8] = include_bin!("svgs/cli/transfer.svg");
        static svg_arrow_medgrey:     &'static [u8] = include_bin!("svgs/buttons/arrow_medgrey.svg");
        static svg_arrow_white:       &'static [u8] = include_bin!("svgs/buttons/arrow_white.svg");
        static svg_attach:            &'static [u8] = include_bin!("svgs/buttons/attach.svg");
        static svg_call:              &'static [u8] = include_bin!("svgs/buttons/call.svg");
        static svg_check:             &'static [u8] = include_bin!("svgs/buttons/check.svg");
        static svg_emoticon:          &'static [u8] = include_bin!("svgs/buttons/emoticon.svg");
        static svg_no:                &'static [u8] = include_bin!("svgs/buttons/no.svg");
        static svg_pause:             &'static [u8] = include_bin!("svgs/buttons/pause.svg");
        static svg_sendmessage:       &'static [u8] = include_bin!("svgs/buttons/sendmessage.svg");
        static svg_video:             &'static [u8] = include_bin!("svgs/buttons/video.svg");
        static svg_online_new:        &'static [u8] = include_bin!("svgs/cli/online_new.svg");
        static svg_offline_new:       &'static [u8] = include_bin!("svgs/cli/offline_new.svg");
        static svg_away_new:          &'static [u8] = include_bin!("svgs/cli/away_new.svg");
        static svg_online:            &'static [u8] = include_bin!("svgs/cli/online.svg");
        static svg_offline:           &'static [u8] = include_bin!("svgs/cli/offline.svg");
        static svg_away:              &'static [u8] = include_bin!("svgs/cli/away.svg");

        macro_rules! create {
            ($path:ident) => {
                {
                    let svg = svg::Handle::from_slice($path).unwrap();
                    let (width, height) = svg.dimensions();
                    let mut surface = cairo::recording_surface_create(cairo::ContentColorAlpha);
                    svg.render_cairo(surface.create());
                    (surface, width, height)
                }
            }
        };
        Assets {
            add:               create!(svg_add),
            group:             create!(svg_group),
            group_large:       create!(svg_group_large),
            group_large_black: create!(svg_group_large_black),
            settings:          create!(svg_settings),
            transfer:          create!(svg_transfer),
            arrow_medgrey:     create!(svg_arrow_medgrey),
            arrow_white:       create!(svg_arrow_white),
            attach:            create!(svg_attach),
            call:              create!(svg_call),
            check:             create!(svg_check),
            emoticon:          create!(svg_emoticon),
            no:                create!(svg_no),
            pause:             create!(svg_pause),
            sendmessage:       create!(svg_sendmessage),
            video:             create!(svg_video),
            online_new:        create!(svg_online_new),
            offline_new:       create!(svg_offline_new),
            away_new:          create!(svg_away_new),
            online:            create!(svg_online),
            offline:           create!(svg_offline),
            away:              create!(svg_away),
        }
    }

    pub fn cache<'b>(&self, scale: f64) -> Cached<'b> {
        macro_rules! create {
            ($pre:expr) => {
                {
                    let &(ref s, ref w, ref h) = &$pre;
                    let mut surface = cairo::image_surface_create(cairo::FormatArgb32,
                                                                  (w*scale) as i32,
                                                                  (h*scale) as i32);
                    {
                        let mut cx = surface.create();
                        cx.scale(scale, scale);
                        cx.set_source_surface(s, 0.0, 0.0);
                        cx.paint();
                    }
                    surface
                }
            }
        };
        Cached {
            add:               create!(self.add),
            group:             create!(self.group),
            group_large:       create!(self.group_large),
            group_large_black: create!(self.group_large_black),
            settings:          create!(self.settings),
            transfer:          create!(self.transfer),
            arrow_medgrey:     create!(self.arrow_medgrey),
            arrow_white:       create!(self.arrow_white),
            attach:            create!(self.attach),
            call:              create!(self.call),
            check:             create!(self.check),
            emoticon:          create!(self.emoticon),
            no:                create!(self.no),
            pause:             create!(self.pause),
            sendmessage:       create!(self.sendmessage),
            video:             create!(self.video),
            online_new:        create!(self.online_new),
            offline_new:       create!(self.offline_new),
            away_new:          create!(self.away_new),
            online:            create!(self.online),
            offline:           create!(self.offline),
            away:              create!(self.away),
        }
    }
}
