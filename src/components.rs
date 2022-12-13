//! Components includes a Player, Render, Position

use std::collections::HashSet;

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

/// 敌人随机移动
#[derive(Clone, Copy, PartialEq)]
pub struct MovingRandomly;

/// 移动
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}

/// 血量
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

/// npc名字
#[derive(Debug, Clone, PartialEq)]
pub struct Name(pub String);

/// 攻击
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub victim: Entity,
}

/// 追赶玩家(野怪智能攻击玩家)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChasingPlayer;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Item;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AmuletOfYala;

#[derive(Debug, Clone, PartialEq)]
pub struct FieldOfView {
    pub visible_tiles: HashSet<Point>,
    pub radius: i32,
    pub is_dirty: bool,
}

impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius,
            is_dirty: true,
        }
    }

    pub fn clone_dirty(&self) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius: self.radius,
            is_dirty: true,
        }
    }
}
