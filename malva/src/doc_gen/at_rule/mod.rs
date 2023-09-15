use super::super::DocGen;
use crate::ctx::Ctx;
use raffia::ast::*;
use tiny_pretty::Doc;

mod container;
mod import;
mod media;
mod supports;

impl<'s> DocGen<'s> for AtRule<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        let mut docs = Vec::with_capacity(5);
        docs.push(Doc::text(format!(
            "@{}",
            self.name.raw.to_ascii_lowercase()
        )));
        if let Some(prelude) = &self.prelude {
            docs.push(Doc::space());
            docs.push(prelude.doc(ctx));
        }
        if let Some(block) = &self.block {
            docs.push(Doc::space());
            docs.push(block.doc(ctx));
        }
        Doc::list(docs)
    }
}

impl<'s> DocGen<'s> for AtRulePrelude<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match self {
            AtRulePrelude::Media(media) => media.doc(ctx),
            AtRulePrelude::Charset(charset) => charset.doc(ctx),
            AtRulePrelude::ColorProfile(color_profile) => color_profile.doc(ctx),
            AtRulePrelude::Container(container) => container.doc(ctx),
            AtRulePrelude::CounterStyle(counter_style) => counter_style.doc(ctx),
            AtRulePrelude::CustomMedia(custom_media) => custom_media.doc(ctx),
            AtRulePrelude::Document(document) => document.doc(ctx),
            AtRulePrelude::FontFeatureValues(font_feature_values) => font_feature_values.doc(ctx),
            AtRulePrelude::FontPaletteValues(font_palette_values) => font_palette_values.doc(ctx),
            AtRulePrelude::Import(import) => import.doc(ctx),
            AtRulePrelude::Keyframes(keyframes) => keyframes.doc(ctx),
            AtRulePrelude::Layer(layer) => layer.doc(ctx),
            AtRulePrelude::Namespace(namespace) => namespace.doc(ctx),
            AtRulePrelude::Nest(nest) => nest.doc(ctx),
            AtRulePrelude::Page(page) => page.doc(ctx),
            AtRulePrelude::PositionFallback(position_fallback) => position_fallback.doc(ctx),
            AtRulePrelude::Property(property) => property.doc(ctx),
            AtRulePrelude::SassExpr(sass_expr) => sass_expr.doc(ctx),
            AtRulePrelude::ScrollTimeline(scroll_timeline) => scroll_timeline.doc(ctx),
            AtRulePrelude::Supports(supports) => supports.doc(ctx),
            _ => todo!(),
        }
    }
}

impl<'s> DocGen<'s> for ColorProfilePrelude<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match self {
            ColorProfilePrelude::DashedIdent(dashed_ident) => dashed_ident.doc(ctx),
            ColorProfilePrelude::DeviceCmyk(device_cmyk) => device_cmyk.doc(ctx),
        }
    }
}

impl<'s> DocGen<'s> for CustomMedia<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        self.name
            .doc(ctx)
            .append(Doc::space())
            .append(self.value.doc(ctx))
    }
}

impl<'s> DocGen<'s> for CustomMediaValue<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match self {
            CustomMediaValue::MediaQueryList(media_query_list) => media_query_list.doc(ctx),
            CustomMediaValue::True(..) => Doc::text("true"),
            CustomMediaValue::False(..) => Doc::text("false"),
        }
    }
}

impl<'s> DocGen<'s> for DocumentPrelude<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::list(
            itertools::intersperse(
                self.matchers.iter().map(|matcher| matcher.doc(ctx)),
                Doc::text(",").append(Doc::line_or_space()),
            )
            .collect(),
        )
        .group()
        .nest(ctx.indent_width)
    }
}

impl<'s> DocGen<'s> for DocumentPreludeMatcher<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match self {
            DocumentPreludeMatcher::Function(function) => function.doc(ctx),
            DocumentPreludeMatcher::Url(url) => url.doc(ctx),
        }
    }
}

impl<'s> DocGen<'s> for FontFamilyName<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match self {
            FontFamilyName::Str(str) => str.doc(ctx),
            FontFamilyName::Unquoted(unquoted) => unquoted.doc(ctx),
        }
    }
}

impl<'s> DocGen<'s> for KeyframeBlock<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        use crate::config::BlockSelectorLineBreak;

        Doc::list(
            itertools::intersperse(
                self.selectors.iter().map(|selector| selector.doc(ctx)),
                Doc::text(",").append(match ctx.options.block_selector_linebreak {
                    BlockSelectorLineBreak::Always => Doc::hard_line(),
                    BlockSelectorLineBreak::Consistent => Doc::line_or_space(),
                    BlockSelectorLineBreak::Wrap => Doc::soft_line(),
                }),
            )
            .collect(),
        )
        .group()
        .append(Doc::space())
        .append(self.block.doc(ctx))
    }
}

impl<'s> DocGen<'s> for KeyframesName<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match self {
            KeyframesName::Ident(ident) => ident.doc(ctx),
            KeyframesName::Str(str) => str.doc(ctx),
            KeyframesName::LessVariable(less_variable) => less_variable.doc(ctx),
            KeyframesName::LessEscapedStr(less_escaped_str) => less_escaped_str.doc(ctx),
        }
    }
}

impl<'s> DocGen<'s> for KeyframeSelector<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match self {
            KeyframeSelector::Percentage(percentage) => percentage.doc(ctx),
            KeyframeSelector::Ident(InterpolableIdent::Literal(Ident { name, .. }))
                if name.eq_ignore_ascii_case("from") =>
            {
                Doc::text("from")
            }
            KeyframeSelector::Ident(InterpolableIdent::Literal(Ident { name, .. }))
                if name.eq_ignore_ascii_case("to") =>
            {
                Doc::text("to")
            }
            KeyframeSelector::Ident(ident) => ident.doc(ctx),
        }
    }
}

impl<'s> DocGen<'s> for LayerName<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::list(
            itertools::intersperse(
                self.idents.iter().map(|ident| ident.doc(ctx)),
                Doc::text("."),
            )
            .collect(),
        )
    }
}

impl<'s> DocGen<'s> for NamespacePrelude<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        if let Some(prefix) = &self.prefix {
            prefix
                .doc(ctx)
                .append(Doc::line_or_space())
                .append(self.uri.doc(ctx))
                .group()
                .nest(ctx.indent_width)
        } else {
            self.uri.doc(ctx)
        }
    }
}

impl<'s> DocGen<'s> for NamespacePreludeUri<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match self {
            NamespacePreludeUri::Str(str) => str.doc(ctx),
            NamespacePreludeUri::Url(url) => url.doc(ctx),
        }
    }
}

impl<'s> DocGen<'s> for PageSelector<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        let pseudo = Doc::list(self.pseudo.iter().map(|pseudo| pseudo.doc(ctx)).collect());
        if let Some(name) = &self.name {
            name.doc(ctx).append(pseudo)
        } else {
            pseudo
        }
    }
}

impl<'s> DocGen<'s> for PageSelectorList<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        use crate::config::BlockSelectorLineBreak;

        Doc::list(
            itertools::intersperse(
                self.selectors.iter().map(|selector| selector.doc(ctx)),
                Doc::text(",").append(match ctx.options.block_selector_linebreak {
                    BlockSelectorLineBreak::Always => Doc::hard_line(),
                    BlockSelectorLineBreak::Consistent => Doc::line_or_space(),
                    BlockSelectorLineBreak::Wrap => Doc::soft_line(),
                }),
            )
            .collect(),
        )
        .group()
        .nest(ctx.indent_width)
    }
}

impl<'s> DocGen<'s> for PseudoPage<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::text(":").append(self.name.doc(ctx))
    }
}

impl<'s> DocGen<'s> for UnquotedFontFamilyName<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::list(
            itertools::intersperse(self.idents.iter().map(|ident| ident.doc(ctx)), Doc::space())
                .collect(),
        )
    }
}
