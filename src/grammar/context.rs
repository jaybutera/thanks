use colored::Colorize;
use std::{
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
    path::Path,
    sync::Arc,
};

use std::path::PathBuf;
use internment::Intern;
//use once_cell::sync::Lazy;

pub type FileId = Intern<String>;

pub type CtxResult<T> = Result<T, CtxErr>;

pub trait ToCtxErr {
    type Okay;
    fn err_ctx(self, loc: Option<CtxLocation>) -> Result<Self::Okay, CtxErr>;
}

impl<T, E: Into<anyhow::Error>> ToCtxErr for Result<T, E> {
    type Okay = T;
    fn err_ctx(self, loc: Option<CtxLocation>) -> Result<Self::Okay, CtxErr> {
        self.map_err(|e| Ctx {
            inner: Arc::new(e.into()),
            context: loc,
        })
    }
}

pub type CtxErr = Ctx<anyhow::Error>;

pub trait ToCtx: Sized {
    fn with_ctx(self, loc: impl Into<Option<CtxLocation>>) -> Ctx<Self> {
        Ctx {
            inner: Arc::new(self),
            context: loc.into(),
        }
    }
}

impl<T: Sized> ToCtx for T {}

/// Represents a reference-counted value that carries a context.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Ctx<T> {
    inner: Arc<T>,
    context: Option<CtxLocation>,
}

impl<T> Clone for Ctx<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            context: self.context,
        }
    }
}

impl<T: Debug> Debug for Ctx<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl<T> Ctx<T> {
    /// Obtains the context.
    pub fn ctx(&self) -> Option<CtxLocation> {
        self.context
    }
}

impl<T> Deref for Ctx<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl<T: Clone> DerefMut for Ctx<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        Arc::make_mut(&mut self.inner)
    }
}

impl<T> From<T> for Ctx<T> {
    fn from(val: T) -> Self {
        Ctx {
            inner: val.into(),
            context: None,
        }
    }
}

impl<T: Display> Display for Ctx<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut error_location = "".into();
        let mut detailed_line: Option<String> = None;
        if let Some(ctx) = self.ctx() {
            if let Ok(source_full_string) =
                //std::fs::read_to_string(*ctx.source)
                std::fs::read_to_string(Path::new(&ctx.source.to_string()))
            {
                let mut char_counter = 0;
                //let mut errloc: String = *ctx.source.to_str().unwrap().into();
                let mut errloc = "".into();
                for (lineno, line) in source_full_string.split('\n').enumerate() {
                    let line_len = line.len() + 1;
                    if char_counter + line.len() > ctx.start_offset {
                        let line_offset = ctx.start_offset - char_counter;
                        errloc = format!("{}", lineno + 1);
                        detailed_line = Some(format!("{}\n{}", line, {
                            let mut toret = String::new();
                            for _ in 0..line_offset {
                                toret.push(' ');
                            }
                            toret.push_str(&format!("{}", "^".bright_green().bold()));
                            for _ in
                                1..(ctx.end_offset - ctx.start_offset).min(line.len() - line_offset)
                            {
                                toret.push_str(&format!("{}", "~".bright_green().bold()));
                            }
                            toret
                        }));
                        break;
                    }
                    char_counter += line_len
                }
                error_location = errloc;
            } //else {
                //error_location = ctx.source.to_string();
            //}
        } else {
            error_location = "(unknown location)".to_string();
        }

        let err_str = format!(
            "{}: {} {}",
            error_location.bold(),
            "error:".bold().red(),
            self.inner.to_string().bold()
        );

        if let Some(line) = detailed_line {
            let lines = line.lines().collect::<Vec<&str>>().join("\n\t");
            std::fmt::Display::fmt(&format!("{}\n\t{}", err_str, lines), f)
        } else {
            std::fmt::Display::fmt(&err_str, f)
        }
    }
}

/// Represents an input location.
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug)]
pub struct CtxLocation {
    pub source: FileId,
    pub start_offset: usize,
    pub end_offset: usize,
}
