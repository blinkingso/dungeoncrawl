//! Components includes a Player, Render, Position

use crate::prelude::*;

/// 地图, 角色等渲染组件
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Render {
    // 前景色和背景色颜色对
    pub color: ColorPair,
    pub glyph: FontCharType,
}

/// 玩家
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Player;

/// 敌人
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Enemy;

#[derive(Clone, Copy, PartialEq)]
pub struct MovingRandomly;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Name(pub String);
