from models import Base, PerformancePlan, Tag, PerformancePlanSong, PerformancePlanAuthor, PerformancePlanTag

# 增加一个新的performance_plan
def add_new_performance_plan(session: Session):
    new_plan = PerformancePlan(
        performance_plan_name="NewPlan",
        basefolder="",
        dancer_number=1, 
        rating=None,
        comments="New Dance Plan"
    )
    session.add(new_plan)
    session.commit()

# 保存修改的performance_plan
def save_performance_plan(session: Session, performance_plan_data):
    try:
        # 检查是否提供了 plan_id，以决定是更新还是创建新条目
        if 'plan_id' in performance_plan_data and performance_plan_data['plan_id']:
            # 更新现有的 PerformancePlan
            plan = session.query(PerformancePlan).filter_by(id=performance_plan_data['plan_id']).first()
            if plan:
                for key, value in performance_plan_data.items():
                    if hasattr(plan, key):
                        setattr(plan, key, value)
            else:
                raise ValueError("PerformancePlan with plan_id {} not found".format(performance_plan_data['plan_id']))
        else:
            # 创建一个新的 PerformancePlan
            new_plan = PerformancePlan(**performance_plan_data)
            session.add(new_plan)
        
        # 提交更改
        session.commit()
        return "Performance Plan saved successfully."
    except SQLAlchemyError as e:
        session.rollback()
        return f"An error occurred: {e}"
    except ValueError as e:
        return str(e)
        
def delete_performance_plan(session: Session, plan_id: int):
    try:
        # 查询是否存在该 PerformancePlan
        plan = session.query(PerformancePlan).filter(PerformancePlan.id == plan_id).first()
        if not plan:
            raise ValueError("Performance Plan not found")

        # 删除找到的 PerformancePlan
        session.delete(plan)
        session.commit()
        print("Performance Plan deleted successfully.")
    except ValueError as e:
        # 捕获没有找到的异常并向调用者报告
        print(e)
        session.rollback()
    except SQLAlchemyError as e:
        # 捕获其他 SQLAlchemy 错误
        print("Failed to delete Performance Plan:", e)
        session.rollback()