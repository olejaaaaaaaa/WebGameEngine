




extern crate rapier2d;
use rapier2d::prelude::*;



pub struct Physics {
    phys_pipeline: PhysicsPipeline,
    phys_setting: PhysicsSetting,
    ball_body_handle: RigidBodyHandle,
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
}

pub struct PhysicsSetting {
    integration_params: IntegrationParameters,
    island_manager: IslandManager,
    broad_phase: Box<BroadPhase>,
    narrow_phase: NarrowPhase,
    impulse_join_set: ImpulseJointSet,
    multi_body_join_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    query_pipeline: QueryPipeline,
}


impl Physics {
    pub fn new() -> Physics {

        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();

        let collider = ColliderBuilder::cuboid(100.0, 0.0).build();
        collider_set.insert(collider);
   

        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![0.0, 10.0])
            .build();

        let collider = ColliderBuilder::ball(0.5).restitution(0.7).build();

        let ball_body_handle = rigid_body_set.insert(rigid_body);
        collider_set.insert_with_parent(collider, ball_body_handle, &mut rigid_body_set);

        let integration_parameters = IntegrationParameters::default();
        let mut physics_pipeline = PhysicsPipeline::new();
        let mut island_manager = IslandManager::new();
        let mut broad_phase = DefaultBroadPhase::new();
        let mut narrow_phase = NarrowPhase::new();
        let mut impulse_joint_set = ImpulseJointSet::new();
        let mut multibody_joint_set = MultibodyJointSet::new();
        let mut ccd_solver = CCDSolver::new();
        let mut query_pipeline = QueryPipeline::new();

        let phys_setting = PhysicsSetting {
            integration_params: integration_parameters,
            island_manager,
            broad_phase: Box::new(broad_phase),
            narrow_phase,
            impulse_join_set: impulse_joint_set,
            multi_body_join_set: multibody_joint_set,
            ccd_solver,
            query_pipeline
        };

        let phys = Physics {
            ball_body_handle,
            phys_setting,
            phys_pipeline: physics_pipeline,
            rigid_body_set,
            collider_set,
        };

        phys
    }

    pub fn update_physics(&mut self) -> (f32, f32){

        let mut gravity = vector![0.0, -9.81];

        self.phys_pipeline.step(
            &mut gravity, 
                &self.phys_setting.integration_params, 
                            &mut self.phys_setting.island_manager, 
                                    &mut *self.phys_setting.broad_phase, 
                                    &mut self.phys_setting.narrow_phase, 
                            &mut self.rigid_body_set, 
                        &mut self.collider_set, 
                    &mut self.phys_setting.impulse_join_set, 
                &mut self.phys_setting.multi_body_join_set, 
                                    &mut self.phys_setting.ccd_solver, 
                    Some(&mut self.phys_setting.query_pipeline), 
                            &(), 
                            &()
        );

        let b = &self.rigid_body_set[self.ball_body_handle];
        let x = b.translation().x;
        let y = b.translation().y;
        
        (x, y - 0.8)
    }
}